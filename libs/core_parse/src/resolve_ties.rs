use core::{context::CoreContext, part::PartType, ties::TieFrom, voice::VoiceType};
use std::error::Error;

type SysitemPosition = usize;
type SysitemId = usize;

#[allow(dead_code, unused_variables, unused_imports)]

pub fn resolve_ties(cx: &CoreContext, partscount: usize) -> Result<(), Box<dyn Error>> {
    let parts = cx.parts.borrow();
    let sysitems = cx.sysitems.borrow();
    // let complexes = cx.complexes.borrow();
    // let partvoices: BTreeMap<(usize, usize), Option<NoteItem>> = BTreeMap::new();

    let sysitems = sysitems
        .iter()
        .filter(|sysitem| matches!(sysitem.stype, core::sysitem::SysItemType::Parts(_, _, _, _)))
        .collect::<Vec<_>>();

    for partidx in 0..partscount {
        let mut partnotes_upper: Vec<(Option<usize>, SysitemPosition, SysitemId)> = Vec::new();
        let mut partnotes_lower: Vec<(Option<usize>, SysitemPosition, SysitemId)> = Vec::new();

        for sysitem in sysitems.iter() {
            // dbg!(sysitem.id, sysitem.position, sysitem.parts_count);
            let part = match &sysitem.stype {
                core::sysitem::SysItemType::Parts(partids, _, _, _) => partids.get(partidx),
                _ => continue,
            };
            match part {
                Some(part_id) => {
                    let part = &parts[*part_id];
                    match &part.ptype {
                        PartType::OneVoice(ref voice) => match &voice.vtype {
                            VoiceType::NoteIds(ref note_ids, _, _) => {
                                // println!("partidx:{partidx} note_ids single voice:{:?}", note_ids);
                                for note_id in note_ids {
                                    partnotes_upper.push((Some(*note_id), sysitem.position, sysitem.id));
                                }
                            }
                            VoiceType::Barpause => {
                                // println!("partidx:{partidx} Handle barpause in resolve_ties");
                                partnotes_upper.push((None, sysitem.position, sysitem.id));
                            }
                        },
                        PartType::TwoVoice(ref voice_upper, ref voice_lower) => {
                            match &voice_upper.vtype {
                                VoiceType::NoteIds(ref note_ids, _, _) => {
                                    // println!("partidx:{partidx} note_ids upper:{:?}", note_ids);
                                    for note_id in note_ids {
                                        partnotes_upper.push((Some(*note_id), sysitem.position, sysitem.id));
                                    }
                                }
                                VoiceType::Barpause => {
                                    // println!("partidx:{partidx} Handle barpause in resolve_ties");
                                    partnotes_upper.push((None, sysitem.position, sysitem.id));
                                }
                            }
                            match &voice_lower.vtype {
                                VoiceType::NoteIds(ref note_ids, _, _) => {
                                    // println!("partidx:{partidx} note_ids lower:{:?}", note_ids);
                                    for note_id in note_ids {
                                        partnotes_lower.push((Some(*note_id), sysitem.position, sysitem.id));
                                    }
                                }
                                VoiceType::Barpause => {
                                    // println!("partidx:{partidx} Handle barpause in resolve_ties");
                                    partnotes_lower.push((None, sysitem.position, sysitem.id));
                                }
                            }
                        }
                        _ => {
                            todo!("Unhandled part type for part_idx: {}", partidx);
                        }
                    }
                }
                None => {
                    //
                    // eprintln!("No part found for index: {}", partidx);
                    partnotes_upper.push((None, sysitem.position, sysitem.id));
                }
            }
        }
        // dbg!(&partnotes_upper);
        // dbg!(&partnotes_lower);

        println!("Resolve upper notes");
        handle_partnotes(cx, partidx, &partnotes_upper)?;
        println!("Resolve lower notes");
        handle_partnotes(cx, partidx, &partnotes_lower)?;
    }

    Ok(())
}

#[allow(dead_code, unused_variables, unused_imports)]

fn handle_partnotes(cx: &CoreContext, partidx: usize, partnotes: &Vec<(Option<usize>, SysitemPosition, SysitemId)>) -> Result<(), Box<dyn Error>> {
    for item in partnotes.windows(2) {
        let left = item[0];
        let right = item[1];
        // println!("left:{:?}, right:{:?}", left, right);

        match (left, right) {
            ((Some(left_id), left_pos, left_sysid), (Some(right_id), right_pos, right_sysid)) if left_sysid == right_sysid => {
                handle_tie_pair(cx, left_id, right_id, left_pos, right_pos, left_sysid, right_sysid)?;
            }
            ((Some(left_id), left_pos, left_sysid), (Some(right_id), right_pos, right_sysid)) => {
                handle_tie_pair(cx, left_id, right_id, left_pos, right_pos, left_sysid, right_sysid)?;
            }
            ((Some(left_id), left_pos, left_sysid), (None, right_pos, right_sysid)) => {
                //
            }
            ((None, left_pos, left_sysid), (Some(right_id), right_pos, right_sysid)) => {
                //
            }
            ((None, _, _), (None, _, _)) => {
                // Both are None, no action needed
            }
        }

        // if let (Some(left_note_id), Some(right_note_id)) = (left.0, right.0) {
        //     if left.1 == right.1 && left.2 == right.2 {
        //         // Tie detected
        //         // Here you would handle the tie logic, e.g., updating the complex or note items
        //         println!(
        //             "Tie detected between note {} and note {} at position {} in system {}",
        //             left_note_id, right_note_id, left.1, left.2
        //         );
        //     }
        // }
    }

    Ok(())
}

fn handle_tie_pair(
    cx: &CoreContext,
    left_id: usize,
    right_id: usize,
    left_pos: SysitemPosition,
    right_pos: SysitemPosition,
    left_sysid: SysitemId,
    right_sysid: SysitemId,
) -> Result<(), Box<dyn Error>> {
    // Here you would implement the logic to handle a tie pair
    // For example, updating the complex or note items
    println!("-----------------------------------");
    if left_sysid != right_sysid {
        println!("Different SYSITEMs: Left and right sysitem IDs do not match: {} != {}", left_sysid, right_sysid);
    }

    println!("Check handling tie pair between note {} and note {} ", left_id, right_id);

    let tiesfrom = cx.map_noteid_tiesfrom.borrow();
    let notes = cx.notes.borrow();

    if tiesfrom.contains_key(&left_id) {
        println!("Left id has tie from!");

        let levels_from = tiesfrom.get(&left_id).unwrap();
        let left_note = &notes[left_id];
        let right_note = &notes[right_id];

        if (left_pos + left_note.position + (left_note.duration as usize)) < right_pos + right_note.position {
            println!("Left note ends before right note starts, no tie possible.");
            // store unresolved tie from left_id
            return Ok(());
        }

        if let Some(levels_to) = right_note.get_head_levels() {
            for level_from in levels_from {
                match level_from {
                    TieFrom::Level(left_level) => {
                        if levels_to.contains(left_level) {
                            println!("Tie detected between note {} and note {} at level {}", left_id, right_id, left_level);
                            // Store resolved from left_id and to right_id
                        } else {
                            println!("No tie found for level: {}", left_level);
                        }
                    }
                }
            }
        }
    } else {
        println!("Nothing to see here");
    }

    Ok(())
}
