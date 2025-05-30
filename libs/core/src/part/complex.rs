use std::collections::BTreeMap;

use crate::{
    context::CoreContext,
    note::{NoteItem, NoteType},
    voice::VoiceType,
    ItemId,
};

use super::PartType;

#[derive(Debug)]
pub struct ComplexItem {
    pub id: usize,
    pub part_id: usize,
    pub position: usize,
    pub duration: usize,
    pub ctype: ComplexType,
}

#[derive(Debug)]
pub enum ComplexType {
    UpperAndLower(NoteItem, NoteItem, i8),
    Upper(NoteItem),
    Lower(NoteItem),
}

/*
#[cfg(test)]
mod tests {

    use crate::context::Context;
    use parse::parse_part;

    #[test]
    fn test_p() {
        let cx = Context::new();
        let _ = parse_part(cx, "d8 2 2 0 % d2 3 d4 3").unwrap();
        dbg!(&cx.stemitems);
        // create_complexes_for_part(cx, part_id);
    }
}
    */

pub fn create_complexes_for_part(cx: &CoreContext, ptype: &PartType, part_id: ItemId) {
    match ptype {
        PartType::OneVoice(ref voice_item) => {
            //
            match voice_item.vtype {
                VoiceType::NoteIds(ref note_ids, duration, _) => {
                    create_complexes_for_one_voice(cx, note_ids, duration, true, part_id);
                }
                _ => {
                    todo!();
                }
            }
        }
        PartType::TwoVoice(ref voice_item_upper, ref voice_item_lower) => {
            //
            match (&voice_item_upper.vtype, &voice_item_lower.vtype) {
                //----------------------------------------------
                // for both upper and lower music voices
                (VoiceType::NoteIds(ref note_ids_upper, duration_upper, _), VoiceType::NoteIds(ref note_ids_lower, duration_lower, _)) => {
                    create_complexes_for_two_voices(cx, note_ids_upper, note_ids_lower, *duration_upper.max(duration_lower), part_id);
                }
                (VoiceType::Barpause, VoiceType::NoteIds(ref note_ids_lower, duration, _)) => {
                    create_complexes_for_one_voice(cx, note_ids_lower, *duration, false, part_id);
                }
                (VoiceType::NoteIds(ref note_ids_upper, duration, _), VoiceType::Barpause) => {
                    create_complexes_for_one_voice(cx, note_ids_upper, *duration, true, part_id);
                }

                _ => todo!("Invalid voice type"),
            }
        }
        _ => panic!("Invalid part type"),
    }
    // dbg!(&cx.complexes);
}

pub fn create_complexes_for_one_voice(cx: &CoreContext, note_ids: &Vec<ItemId>, part_duration: usize, is_upper_voice: bool, part_id: ItemId) {
    let notes = cx.notes.borrow();
    // let notes_positions = cx.notes_positions.borrow();

    let mut positions: Vec<usize> = note_ids
        .iter()
        .map(|note_id| {
            //
            let note = notes.get(*note_id).unwrap();

            note.position
        })
        .collect::<Vec<_>>();

    positions.push(part_duration);

    let mut map_durations: BTreeMap<usize, usize> = BTreeMap::new();
    let _ = &positions.windows(2).collect::<Vec<_>>().iter().for_each(|w| {
        let start = w[0];
        let end = w[1];
        let duration = end - start;
        map_durations.insert(start, duration);
    });

    for note_id in note_ids {
        // let note_position = notes_positions.get(&note_id).unwrap();

        let note = notes.get(*note_id).unwrap();

        let ctype = if is_upper_voice {
            ComplexType::Upper(notes.get(*note_id).unwrap().clone())
        } else {
            ComplexType::Lower(notes.get(*note_id).unwrap().clone())
        };

        ComplexType::Upper(notes.get(*note_id).unwrap().clone());
        let id = cx.complexes.borrow().len();
        let complex = ComplexItem {
            id,
            part_id,
            position: note.position,
            duration: *map_durations.get(&note.position).unwrap(),
            ctype: ctype,
        };
        cx.complexes.borrow_mut().push(complex);
        cx.map_noteid_complexid.borrow_mut().insert(*note_id, id as ItemId);
    }
}

pub fn create_complexes_for_two_voices(cx: &CoreContext, note_ids_upper: &Vec<ItemId>, note_ids_lower: &Vec<ItemId>, part_duration: usize, part_id: ItemId) {
    let notes = cx.notes.borrow();
    let mut map: BTreeMap<usize, Vec<Option<ItemId>>> = BTreeMap::new();

    for note_id in note_ids_upper {
        let note = notes.get(*note_id).unwrap();
        let note_position = note.position;

        // println!("upper: note_id:{}, note_position:{}", note_id, note_position);

        if !map.contains_key(&note_position) {
            map.insert(note_position, vec![Some(*note_id), None]);
        } else {
            let notes = map.get_mut(&note_position).unwrap();
            notes.push(Some(*note_id));
        }
    }

    for note_id in note_ids_lower {
        let note = notes.get(*note_id).unwrap();
        let note_position = note.position;

        // println!("lower: note_id:{}, note_position:{}", note_id, note_position);

        if !map.contains_key(&note_position) {
            map.insert(note_position, vec![None, Some(*note_id)]);
        } else {
            let notes = map.get_mut(&note_position).unwrap();
            if notes[1].is_none() {
                notes[1] = Some(*note_id);
            } else {
                notes.push(Some(*note_id));
            }
        }
    }

    // calculate complex durations
    let mut positions: Vec<usize> = map.keys().map(|p| *p).collect::<Vec<_>>();

    positions.push(part_duration);
    positions.sort();

    let mut map_durations: BTreeMap<usize, usize> = BTreeMap::new();
    let _ = &positions.windows(2).collect::<Vec<_>>().iter().for_each(|w| {
        let start = w[0];
        let end = w[1];

        let duration = end - start;
        map_durations.insert(start, duration);
    });

    //----------------------------------------------------------------------
    // create complex types
    for (position, note_ids) in &map {
        let ctype = match note_ids.as_slice() {
            [Some(note_upper_id), Some(note_lower_id)] => {
                let note_upper = notes.get(*note_upper_id).unwrap();
                let note_lower = notes.get(*note_lower_id).unwrap();

                // check for note head collisions...
                let level_diff: i8 = match (&note_upper.ntype, &note_lower.ntype) {
                    (NoteType::Heads(heads_upper), NoteType::Heads(heads_lower)) => {
                        // dbg!("Check for note head collisions...", &heads_upper, &heads_lower);
                        heads_lower.first().unwrap().level - heads_upper.last().unwrap().level
                        // dbg!(level_diff);
                    }
                    _ => 0,
                };

                ComplexType::UpperAndLower(note_upper.clone(), note_lower.clone(), level_diff)
            }
            [Some(note_upper_id), None] => {
                let note_upper = notes.get(*note_upper_id).unwrap();
                ComplexType::Upper(note_upper.clone())
            }
            [None, Some(note_lower_id)] => {
                let note_lower = notes.get(*note_lower_id).unwrap();
                ComplexType::Lower(note_lower.clone())
            }
            _ => todo!("Invalid notes"),
        };

        // store complex in context
        let id = cx.complexes.borrow().len();
        let complex = ComplexItem {
            id,
            part_id,
            position: *position,
            duration: *map_durations.get(position).unwrap(),
            ctype: ctype,
        };

        cx.complexes.borrow_mut().push(complex);
        for note_id in note_ids {
            if let Some(note_id) = note_id {
                cx.map_noteid_complexid.borrow_mut().insert(*note_id, id as ItemId);
            }
        }
    }
}
