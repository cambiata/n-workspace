use core::{context::CoreContext, part::PartType, voice::VoiceType};
use std::{collections::BTreeMap, error::Error};

pub fn resolve_ties(cx: &CoreContext) -> Result<(), Box<dyn Error>> {
    let parts = cx.parts.borrow();
    // let sysitems = cx.sysitems.borrow();
    // let complexes = cx.complexes.borrow();
    // let partvoices: BTreeMap<(usize, usize), Option<NoteItem>> = BTreeMap::new();

    let mut partnotes_upper: BTreeMap<usize, Vec<(Option<usize>, usize)>> = BTreeMap::new();
    let mut partnotes_lower: BTreeMap<usize, Vec<(Option<usize>, usize)>> = BTreeMap::new();

    for part in parts.iter() {
        let part_idx = part.idx;
        let part_position = part.position;

        if !partnotes_upper.contains_key(&part_idx) {
            partnotes_upper.insert(part_idx, Vec::new());
        }
        if !partnotes_lower.contains_key(&part_idx) {
            partnotes_lower.insert(part_idx, Vec::new());
        }

        match &part.ptype {
            PartType::OneVoice(ref voice) => match &voice.vtype {
                VoiceType::NoteIds(ref note_ids, _, _) => {
                    println!("part_idx:{part_idx} note_ids single voice:{:?}", note_ids);
                    for note_id in note_ids {
                        partnotes_upper.get_mut(&part_idx).unwrap().push((Some(*note_id), part_position));
                    }
                }
                VoiceType::Barpause => {
                    println!("part_idx:{part_idx} Handle barpause in resolve_ties");
                    partnotes_upper.get_mut(&part_idx).unwrap().push((None, part_position));
                    // Placeholder for barpause
                }
            },
            PartType::TwoVoice(ref voice_upper, ref voice_lower) => {
                //
                match &voice_upper.vtype {
                    VoiceType::NoteIds(ref note_ids, _, _) => {
                        println!("part_idx:{part_idx} note_ids upper:{:?}", note_ids);
                        for note_id in note_ids {
                            partnotes_upper.get_mut(&part_idx).unwrap().push((Some(*note_id), part_position));
                        }
                    }
                    VoiceType::Barpause => {
                        println!("part_idx:{part_idx} Handle barpause in resolve_ties");
                        partnotes_upper.get_mut(&part_idx).unwrap().push((None, part_position));
                    }
                }
                match &voice_lower.vtype {
                    VoiceType::NoteIds(ref note_ids, _, _) => {
                        println!("part_idx:{part_idx} note_ids lower:{:?}", note_ids);
                        for note_id in note_ids {
                            partnotes_lower.get_mut(&part_idx).unwrap().push((Some(*note_id), part_position));
                        }
                    }
                    VoiceType::Barpause => {
                        println!("part_idx:{part_idx} Handle barpause in resolve_ties");
                        partnotes_lower.get_mut(&part_idx).unwrap().push((None, part_position));
                    }
                }
            }
            _ => {
                // Handle other part types if necessary
                eprintln!("Unhandled part type for part_idx: {}", part_idx);
            }
        }
    }

    dbg!(&partnotes_upper);
    dbg!(&partnotes_lower);

    /*
    for part_idx in 0..max_parts_count {
        let part_upper_heads: Vec<HeadItem> = Vec::new();
        let part_lower_heads: Vec<HeadItem> = Vec::new();

        for sysitem_id in &parts_items_ids {
            let sysitem = &sysitems[*sysitem_id];
            match &sysitem.stype {
                core::sysitem::SysItemType::Parts(partids, _, _, _) => {
                    let opt_part = partids.get(part_idx);
                    match opt_part {
                        Some(part_id) => {
                            let part = &parts[*part_id];
                            match &part.ptype {
                                PartType::OneVoice(ref voice) => {
                                    match &voice.vtype {
                                        VoiceType::NoteIds(ref note_ids, _, _) => {
                                            println!("part_idx:{part_idx} note_ids single voice:{:?}", note_ids);
                                        }
                                        VoiceType::Barpause => {
                                            todo!("part_idx:{part_idx} Handle barpause in resolve_ties");
                                        }
                                    }
                                }
                                PartType::TwoVoice(ref voice_upper, ref voice_lower) => {
                                    //
                                    match &voice_upper.vtype {
                                        VoiceType::NoteIds(ref note_ids, _, _) => {
                                            println!("part_idx:{part_idx} note_ids upper:{:?}", note_ids);
                                        }
                                        VoiceType::Barpause => {
                                            println!("part_idx:{part_idx} Handle barpause in resolve_ties");
                                        }
                                    }
                                    match &voice_lower.vtype {
                                        VoiceType::NoteIds(ref note_ids, _, _) => {
                                            println!("part_idx:{part_idx} note_ids lower:{:?}", note_ids);
                                        }
                                        VoiceType::Barpause => {
                                            println!("part_idx:{part_idx} Handle barpause in resolve_ties");
                                        }
                                    }
                                }
                                _ => {
                                    // Handle other part types if necessary
                                    eprintln!("Unhandled part type for part_idx: {}", part_idx);
                                }
                            }
                        }
                        None => eprintln!("No part found for index: {}", part_idx),
                    }
                }
                _ => {}
            }
        }
    }
    */

    Ok(())
}
