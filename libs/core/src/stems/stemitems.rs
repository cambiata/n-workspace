use crate::{
    constants::STEM_DEFAULT_LENGTH,
    context::CoreContext,
    direction::DirectionUD,
    duration::{DurationUtils, NoteDuration, SumDuration},
    note::{self, NoteId, NoteItem},
};

pub type StemItemId = usize;

#[derive(Debug)]
pub struct StemItem {
    pub id: StemItemId,
    pub stype: StemType,
    pub direction: Option<DirectionUD>,
    pub position: usize,
    pub duration: usize,
}

#[derive(Debug)]
pub enum StemType {
    NotNote(StemNoteItem),
    NoteWithoutStem(StemNoteItem),
    NoteWithStem(StemNoteItem),
    NotesBeamed(Vec<StemNoteItem>, Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct StemNoteItem {
    pub top_level: i8,
    pub bottom_level: i8,
    pub note: NoteItem,
    pub head_positions: Option<Vec<StemHeadPosition>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StemHeadPosition {
    Left = -1,
    Center = 0,
    Right = 1,
}

pub struct StemItemUtils;
impl StemItemUtils {
    pub fn create_stem_items_from_notes(
        cx: &CoreContext,
        note_ids: &Vec<StemItemId>,
        notes_duration: SumDuration,
        pattern_values: Vec<NoteDuration>,
    ) -> Result<Vec<StemItemId>, Box<dyn std::error::Error>> {
        let groups = StemItemUtils::create_groups_of_notes(cx, note_ids, notes_duration, pattern_values);

        let mut ids: Vec<StemItemId> = Vec::new();

        #[allow(unused_assignments)]
        let mut position: usize = 0;
        let mut duration: usize = 0;
        for group in groups.iter() {
            let stem_item_id = cx.stemitems.borrow().len() as StemItemId;
            let t: Result<StemType, Box<dyn std::error::Error>> = match group.len() {
                // no group of zero notes
                0 => todo!("Should not happen"),

                // group of one note
                1 => {
                    let note = group.get(0).unwrap();
                    position = note.position;
                    duration = note.duration as usize;

                    // Store the stem item id for the note
                    cx.map_noteid_stemitemid.borrow_mut().insert(note.id, stem_item_id);

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

                    let base_values = group.iter().map(|n| n.duration.get_base_value()).collect::<Vec<_>>();

                    for note in group.iter() {
                        // Store the stem item id for the note
                        cx.map_noteid_stemitemid.borrow_mut().insert(note.id, stem_item_id);

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
                    Ok(StemType::NotesBeamed(infos, base_values))
                }
            };
            let t = t?;

            let item = StemItem {
                id: stem_item_id,
                stype: t,
                direction: None,
                position,
                duration,
            };
            cx.stemitems.borrow_mut().push(item);
            ids.push(stem_item_id);
        }
        Ok(ids)
    }

    pub fn create_groups_of_notes(cx: &CoreContext, note_ids: &Vec<NoteId>, notes_duration: SumDuration, pattern_values: Vec<NoteDuration>) -> Vec<Vec<NoteItem>> {
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

    pub fn calculate_stem_lengths_for_notes(cx: &CoreContext, stemitem: &StemItem) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(direction) = &stemitem.direction {
            match &stemitem.stype {
                StemType::NoteWithStem(item) => {
                    let _ = calc_stemlengths_single(cx, item, direction)?;
                }
                StemType::NotesBeamed(items, _) => {
                    calc_stemlengths_beamed(cx, items, direction)?;
                }
                _ => {}
            };
        } else {
            return Ok(()); // Skip items without a direction
        }

        Ok(())
    }
}

fn calc_stemlengths_single(cx: &CoreContext, item: &StemNoteItem, direction: &DirectionUD) -> Result<(), Box<dyn std::error::Error>> {
    // Implement logic for calculating stem lengths for beamed notes in the down direction
    let mut stemitemlevels = cx.map_noteid_stemitemlevels.borrow_mut();
    // let note_configs = cx.map_noteid_configuration.borrow();
    // let note_config = note_configs.get(&item.note.id).unwrap();
    // dbg!(note_config);

    let mut top_level = item.top_level as f32;
    let mut bottom_level = item.bottom_level as f32;
    let note_id = item.note.id;
    let smallest_duration = item.note.duration.get_base_value();
    dbg!(smallest_duration);

    match direction {
        DirectionUD::Up => {
            if top_level < -2.0 {
                top_level += 1.0; // Ensure the top level does not go below -5
            }
            stemitemlevels.insert(note_id, (direction.clone(), (top_level - STEM_DEFAULT_LENGTH).min(0.0), bottom_level));
        }
        DirectionUD::Down => {
            if bottom_level > 2.0 {
                bottom_level -= 1.0; // Ensure the bottom level does not go above 5
            }
            stemitemlevels.insert(note_id, (direction.clone(), top_level, (bottom_level + STEM_DEFAULT_LENGTH).max(0.0)));
        }
    }

    Ok(())
}

fn calc_stemlengths_beamed(cx: &CoreContext, items: &[StemNoteItem], direction: &DirectionUD) -> Result<(), Box<dyn std::error::Error>> {
    // Implement logic for calculating stem lengths for beamed notes in the down direction
    let mut stemitemlevels = cx.map_noteid_stemitemlevels.borrow_mut();

    let mut first_top_level: f32 = items.first().map_or(0.0, |item| item.top_level as f32);
    let mut first_bottom_level: f32 = items.first().map_or(0.0, |item| item.bottom_level as f32);
    let first_note_id: NoteId = items.first().map_or(0, |item| item.note.id);

    let mut last_bottom_level: f32 = items.last().map_or(0.0, |item| item.bottom_level as f32);
    let mut last_top_level: f32 = items.last().map_or(0.0, |item| item.top_level as f32);
    let last_note_id: NoteId = items.last().map_or(0, |item| item.note.id);

    let smallest_base_value = DurationUtils::durations_smallest_base_value(&items.iter().map(|i| i.note.duration).collect::<Vec<_>>());

    match items.len() {
        0 => return Ok(()), // No items to process
        1 => return Ok(()), // No items to process
        _ => {
            // For two notes, calculate the stem length based on the top and bottom levels

            match direction {
                DirectionUD::Up => {
                    // max angle = +-1
                    let top_diff = first_top_level - last_top_level;
                    match top_diff {
                        _ if top_diff > 1.0 => {
                            first_top_level = last_top_level + 1.0;
                        }
                        _ if top_diff < -1.0 => {
                            last_top_level = first_top_level + 1.0;
                        }
                        _ => {}
                    }

                    // compensate for 16ths and 32nds
                    match smallest_base_value {
                        16 => {
                            first_top_level = first_top_level - 1.0;
                            last_top_level = last_top_level - 1.0;
                        }
                        32 => {
                            first_top_level = first_top_level - 2.0;
                            last_top_level = last_top_level - 2.0;
                        }
                        _ => {}
                    }

                    // Ensure the top level does not go below 0
                    first_top_level = (first_top_level - STEM_DEFAULT_LENGTH).min(0.0);
                    last_top_level = (last_top_level - STEM_DEFAULT_LENGTH).min(0.0);

                    // if 3 or more notes, adjust the top levels
                    if items.len() > 2 {
                        let lowest_top_level = first_top_level.max(last_top_level);

                        for miditem in items.iter().skip(1).take(items.len() - 2) {
                            let mid_top_level = miditem.top_level as f32;
                            let mid_bottom_level = miditem.bottom_level as f32;
                            if mid_top_level - lowest_top_level < 5.0 {
                                first_top_level = mid_top_level - 4.0;
                                last_top_level = mid_top_level - 4.0;
                            }

                            stemitemlevels.insert(miditem.note.id, (direction.clone(), mid_top_level, mid_bottom_level));
                        }
                    }

                    stemitemlevels.insert(first_note_id, (direction.clone(), first_top_level, first_bottom_level));
                    stemitemlevels.insert(last_note_id, (direction.clone(), last_top_level, last_bottom_level));
                }
                DirectionUD::Down => {
                    // For downward stem, the bottom level is the lowermost note
                    let bottom_diff = first_bottom_level - last_bottom_level;
                    match bottom_diff {
                        _ if bottom_diff < 1.0 => {
                            first_bottom_level = last_bottom_level - 1.0;
                        }
                        _ if bottom_diff > -1.0 => {
                            last_bottom_level = first_bottom_level - 1.0;
                        }
                        _ => {}
                    }

                    // compensate for 16ths and 32nds
                    match smallest_base_value {
                        16 => {
                            first_bottom_level = first_bottom_level + 1.0;
                            last_bottom_level = last_bottom_level + 1.0;
                        }
                        32 => {
                            first_bottom_level = first_bottom_level + 2.0;
                            last_bottom_level = last_bottom_level + 2.0;
                        }
                        _ => {}
                    }

                    first_bottom_level = (first_bottom_level + STEM_DEFAULT_LENGTH).max(0.0);
                    last_bottom_level = (last_bottom_level + STEM_DEFAULT_LENGTH).max(0.0);

                    if items.len() > 2 {
                        let lowest_bottom_level = first_bottom_level.max(last_bottom_level);

                        for miditem in items.iter().skip(1).take(items.len() - 2) {
                            let mid_top_level = miditem.top_level as f32;
                            let mid_bottom_level = miditem.bottom_level as f32;
                            dbg!(lowest_bottom_level, mid_bottom_level);

                            if lowest_bottom_level - mid_bottom_level < 5.0 {
                                first_bottom_level = mid_bottom_level + 4.0;
                                last_bottom_level = mid_bottom_level + 4.0;
                            }
                            stemitemlevels.insert(miditem.note.id, (direction.clone(), mid_top_level, mid_bottom_level));
                        }
                    }

                    stemitemlevels.insert(first_note_id, (direction.clone(), first_top_level, first_bottom_level));
                    stemitemlevels.insert(last_note_id, (direction.clone(), last_top_level, last_bottom_level));
                }
            }
        }
    }

    Ok(())
}
