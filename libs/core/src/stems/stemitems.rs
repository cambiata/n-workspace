use crate::{
    context::CoreContext,
    direction::DirectionUD,
    duration::{NoteDuration, SumDuration},
    note::{self, NoteItem},
    ItemId,
};

#[derive(Debug)]
pub struct StemItem {
    pub id: ItemId,
    pub stype: StemType,
    pub direction: Option<DirectionUD>,
    pub position: usize,
    pub duration: usize,
    // pub direction: DirectionUD,
}

#[derive(Debug)]
pub enum StemType {
    NotNote(StemNoteItem),
    NoteWithoutStem(StemNoteItem),
    NoteWithStem(StemNoteItem),
    NotesBeamed(Vec<StemNoteItem>),
}

#[derive(Debug, Clone)]
pub struct StemNoteItem {
    // pub note_id: ItemId,
    pub top_level: i8,
    pub bottom_level: i8,
    // pub position: usize,
    // pub duration: Duration,
    pub note: NoteItem,
    pub head_positions: Option<Vec<StemHeadPosition>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StemHeadPosition {
    Left = -1,
    Center = 0,
    Right = 1,
}

/*

#[cfg(test)]
mod tests {
    use crate::context::parsex::parse_part;
    use crate::context::Context;
    #[test]
    fn test_p() {
        let cx = Context::new();
        let _ = parse_part(cx, "d8 2 2 % 0").unwrap();
        dbg!(&cx.stemitems);
    }
}
*/

pub fn create_stem_items_from_notes_in_voice(
    cx: &CoreContext,
    note_ids: &Vec<ItemId>,
    notes_duration: SumDuration,
    pattern_values: Vec<NoteDuration>,
) -> Result<Vec<ItemId>, Box<dyn std::error::Error>> {
    let groups = create_groups_of_notes(cx, note_ids, notes_duration, pattern_values);

    let mut ids: Vec<ItemId> = Vec::new();

    #[allow(unused_assignments)]
    let mut position: usize = 0;
    let mut duration: usize = 0;
    for group in groups.iter() {
        let t: Result<StemType, Box<dyn std::error::Error>> = match group.len() {
            // no group of zero notes
            0 => todo!("Should not happen"),

            // group of one note
            1 => {
                let note = group.get(0).unwrap();
                position = note.position;
                duration = note.duration as usize;
                match note.ntype {
                    note::NoteType::Heads(ref heads) => match note.has_stem() {
                        //
                        true => Ok(StemType::NoteWithStem(StemNoteItem {
                            // note_id: note.id,
                            top_level: heads.first().unwrap().level,
                            bottom_level: heads.last().unwrap().level,
                            // position: note.position,
                            // duration: note.duration,
                            note: note.clone(),
                            head_positions: None,
                        })),
                        false => Ok(StemType::NoteWithoutStem(StemNoteItem {
                            // note_id: note.id,
                            top_level: heads.first().unwrap().level,
                            bottom_level: heads.last().unwrap().level,
                            // position: note.position,
                            // duration: note.duration,
                            note: note.clone(),
                            head_positions: None,
                        })),
                    },
                    _ => Ok(StemType::NotNote(StemNoteItem {
                        top_level: 0,
                        bottom_level: 0,
                        note: note.clone(),
                        head_positions: None,
                    })),
                }
            }

            // group of two or more notes
            _ => {
                let mut infos: Vec<StemNoteItem> = Vec::new();

                position = group.first().unwrap().position;

                for note in group.iter() {
                    match note.ntype {
                        note::NoteType::Heads(ref heads) => {
                            let info = StemNoteItem {
                                // note_id: note.id,
                                top_level: heads.first().unwrap().level,
                                bottom_level: heads.last().unwrap().level,
                                // position: note.position,
                                // duration: note.duration,
                                note: note.clone(),
                                head_positions: None,
                            };
                            infos.push(info);
                        }
                        _ => {
                            todo!("Should not happen");
                        }
                    }
                    duration += note.duration as usize;
                }
                Ok(StemType::NotesBeamed(infos))
            }
        };
        let t = t?;

        let id = cx.stemitems.borrow().len() as ItemId;
        let item = StemItem {
            id,
            stype: t,
            direction: None,
            position,
            duration,
        };
        cx.stemitems.borrow_mut().push(item);
        ids.push(id);
    }
    Ok(ids)
}

pub fn create_groups_of_notes(cx: &CoreContext, note_ids: &Vec<ItemId>, notes_duration: SumDuration, pattern_values: Vec<NoteDuration>) -> Vec<Vec<NoteItem>> {
    let mut cycles: Vec<(usize, usize)> = Vec::new();
    let mut position = 0;
    let mut step = 0;

    // build group cycles
    while position < notes_duration {
        let v = pattern_values[step % pattern_values.len()];
        cycles.push((position, position + v as usize));
        step += 1;
        position += v as usize;
    }

    let cx_notes = cx.notes.borrow();
    let notes = note_ids
        .iter()
        .map(|note_id| {
            let note = cx_notes.get(*note_id).unwrap();
            note
        })
        .collect::<Vec<_>>();

    let mut notes_iterator = notes.into_iter();
    let mut current_note: Option<&NoteItem> = notes_iterator.next();
    let mut cycle_iterator = cycles.iter();
    let mut current_cycle: Option<&(usize, usize)> = cycle_iterator.next();
    let mut loop_safe = 0;
    let mut groups: Vec<Vec<NoteItem>> = Vec::new();
    let mut group: Vec<NoteItem> = Vec::new();

    while current_note.is_some() && current_cycle.is_some() && loop_safe < 100 {
        let note = current_note.unwrap();
        let cycle = current_cycle.unwrap();
        let cycle_start = cycle.0;
        let cycle_end = cycle.1;

        if !note.is_beamable() {
            if !group.is_empty() {
                // println!(" > Flusha grupp");
                groups.push(group);
                group = Vec::new();
            }
            // println!(" > Spapa ny grupp");
            group.push(note.clone());
            groups.push(group);
            group = Vec::new();
            // println!(" - - Ticka fram en ny not");
            current_note = notes_iterator.next();
        } else if note.position >= cycle_start && note.position < cycle_end {
            // println!(" - - notens början är inom cycle-spannet");
            if note.position + note.duration as usize <= cycle_end {
                // println!(" - - notens slut ryms i cycle");
                // println!(" > Lägg till i grupp");
                group.push(note.clone());
                // println!(" - - Ticka fram en ny not");
                current_note = notes_iterator.next();
            } else {
                // println!(" - - notens slut ryms inte i cycle");
                // println!(" > Skapa ny grupp");
                if !group.is_empty() {
                    groups.push(group)
                };
                group = Vec::new();
                // println!(" - - Ticka fram ny cycle");
                current_cycle = cycle_iterator.next();
            }
            // current_note = notes_iterator.next();
        } else {
            // println!(" - - notens början är inte inom cycle spannet");
            if note.position < cycle_start {
                // println!(" - - notens position är FÖRE cycle_start");
                // println!(" - - Ticka fram en ny not");
                current_note = notes_iterator.next();
            } else {
                // println!(" - - notens position är EFTER cycle_start");
                // println!(" > Skapa ny grupp");
                if !group.is_empty() {
                    groups.push(group)
                };
                group = Vec::new();
                // println!(" - - ticka fram cycle");
                current_cycle = cycle_iterator.next();
            }
        }
        loop_safe += 1;
    }
    if !group.is_empty() {
        groups.push(group);
    }
    groups
}
