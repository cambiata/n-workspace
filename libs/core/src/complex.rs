use std::collections::BTreeMap;

use crate::{
    context::CoreContext,
    direction::DirectionUD,
    note::{NoteId, NoteItem, NoteType},
    part::{PartId, PartType},
    voice::VoiceType,
};

pub type ComplexId = usize;

#[derive(Debug)]
pub struct Complex {
    pub id: ComplexId,
    pub part_id: usize,
    pub position: usize,
    pub duration: usize,
    pub position2: Option<usize>,
    pub ctype: ComplexType,
    pub offsets: ComplexHeadOffsets,
}

#[derive(Debug)]
pub enum ComplexType {
    UpperAndLower(NoteItem, NoteItem, i8),
    Upper(NoteItem),
    Lower(NoteItem),
}

#[derive(Debug)]
pub enum ComplexHeadOffsets {
    None,
    UpperX(f32),
    LowerX(f32),
    UpperLowerX(f32, f32),
}

#[derive(Debug, Clone)]
pub enum ComplexConfiguration {
    OneNote,
    TwoNotes(DirectionUD),
}

pub type ComplexInfo = (usize, usize, usize);

pub struct ComplexUtils;
impl ComplexUtils {
    pub fn create_complexes_for_part(cx: &CoreContext, ptype: &PartType, part_id: PartId) -> Vec<usize> {
        match ptype {
            PartType::OtherPart => Vec::new(),
            // PartType::Barpause => Vec::new(),
            PartType::OneVoice(ref voice_item) => {
                //
                match voice_item.vtype {
                    VoiceType::NoteIds(ref note_ids, duration, _) => ComplexUtils::create_complexes_for_one_voice(cx, note_ids, duration, true, part_id, 0),
                    _ => {
                        // todo!("Cannot create complexes for barpause");
                        Vec::new()
                    }
                }
            }
            PartType::TwoVoice(ref voice_item_upper, ref voice_item_lower) => {
                //
                match (&voice_item_upper.vtype, &voice_item_lower.vtype) {
                    //----------------------------------------------
                    // for both upper and lower music voices
                    (VoiceType::NoteIds(ref note_ids_upper, duration_upper, _), VoiceType::NoteIds(ref note_ids_lower, duration_lower, _)) => {
                        ComplexUtils::create_complexes_for_two_voices(cx, note_ids_upper, note_ids_lower, *duration_upper.max(duration_lower), part_id, 0)
                    }
                    (VoiceType::Barpause, VoiceType::NoteIds(ref note_ids_lower, duration, _)) => ComplexUtils::create_complexes_for_one_voice(cx, note_ids_lower, *duration, false, part_id, 0),

                    (VoiceType::NoteIds(ref note_ids_upper, duration, _), VoiceType::Barpause) => ComplexUtils::create_complexes_for_one_voice(cx, note_ids_upper, *duration, true, part_id, 0),

                    _ => todo!("Invalid voice type"),
                }
            } // _ => panic!("Invalid part type"),
        }
        // dbg!(&cx.complexes);
    }

    pub fn create_complexes_for_one_voice(cx: &CoreContext, note_ids: &Vec<NoteId>, part_duration: usize, is_upper_voice: bool, part_id: PartId, column_position: usize) -> Vec<usize> {
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

        let mut partid_complexids: Vec<usize> = vec![];
        // let mut partid_complexpositions: Vec<usize> = vec![];
        // let mut partid_complexdurations: Vec<usize> = vec![];

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
            let complex = Complex {
                id,
                part_id,
                position: note.position,
                duration: *map_durations.get(&note.position).unwrap(),
                position2: Some(column_position + note.position),
                ctype: ctype,
                offsets: ComplexHeadOffsets::None,
            };
            cx.map_noteid_complexid.borrow_mut().insert(*note_id, id as ComplexId);
            partid_complexids.push(id);
            // partid_complexpositions.push(complex.position);
            // partid_complexdurations.push(complex.duration);

            cx.complexes.borrow_mut().push(complex);
        }

        // store partid_complexids in context
        // cx.map_partid_complexids.borrow_mut().insert(part_id, partid_complexids);
        partid_complexids
    }

    pub fn create_complexes_for_two_voices(cx: &CoreContext, note_ids_upper: &Vec<NoteId>, note_ids_lower: &Vec<NoteId>, part_duration: usize, part_id: PartId, column_position: usize) -> Vec<usize> {
        let notes = cx.notes.borrow();
        let mut map: BTreeMap<usize, Vec<Option<NoteId>>> = BTreeMap::new();

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
        let mut partid_complexids: Vec<usize> = vec![];
        // let mut partid_complexpositions: Vec<usize> = vec![];
        // let mut partid_complexdurations: Vec<usize> = vec![];

        for (position, note_ids) in &map {
            let ctype = match note_ids.as_slice() {
                [Some(note_upper_id), Some(note_lower_id)] => {
                    let note_upper = notes.get(*note_upper_id).unwrap();
                    let note_lower = notes.get(*note_lower_id).unwrap();

                    // store value for check for note head collisions...
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

            // calculate head offsets to avoid collisions
            let offsets = ComplexUtils::calculate_head_offsets(cx, &ctype);

            // store complex in context
            let id = cx.complexes.borrow().len();
            let complex = Complex {
                id,
                part_id,
                position: *position,
                duration: *map_durations.get(position).unwrap(),
                position2: Some(column_position + *position),
                ctype: ctype,
                offsets,
            };

            for note_id in note_ids {
                if let Some(note_id) = note_id {
                    cx.map_noteid_complexid.borrow_mut().insert(*note_id, id as ComplexId);
                }
            }
            partid_complexids.push(id);
            // partid_complexpositions.push(complex.position);
            // partid_complexdurations.push(complex.duration);
            cx.complexes.borrow_mut().push(complex);
        }
        // store partid_complexids in context
        // cx.map_partid_complexids.borrow_mut().insert(part_id, partid_complexids);
        partid_complexids
    }

    fn calculate_head_offsets(cx: &CoreContext, ctype: &ComplexType) -> ComplexHeadOffsets {
        match ctype {
            ComplexType::UpperAndLower(upper, lower, level_diff) => {
                //store offsets for notes?
                match level_diff {
                    _ if *level_diff <= 0 => {
                        // store offsets for upper
                        let id = upper.id;
                        let offset = -13.0;

                        cx.map_noteid_headoffsetx.borrow_mut().insert(id, offset);
                        ComplexHeadOffsets::UpperX(offset)
                    }
                    _ if *level_diff == 1 => {
                        // store offsets for lower
                        let id = lower.id;
                        let offset = 10.0;
                        cx.map_noteid_headoffsetx.borrow_mut().insert(id, offset);
                        ComplexHeadOffsets::LowerX(offset)
                    }
                    _ => ComplexHeadOffsets::None,
                }
            }
            _ => ComplexHeadOffsets::None,
        }
    }
}
