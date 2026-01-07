use core::{context::CoreContext, part::PartType, sysitem::VecPartNotes, voice::VoiceType};
use std::error::Error;

#[allow(dead_code, unused_variables, unused_imports)]

pub fn create_part_notes_vecs(cx: &CoreContext, partscount: usize) -> Result<Vec<(VecPartNotes, VecPartNotes)>, Box<dyn Error>> {
    let parts = cx.parts.borrow();
    let sysitems = cx.sysitems.borrow();

    let sysitems = sysitems
        .iter()
        .filter(|sysitem| matches!(sysitem.stype, core::sysitem::SysItemType::Parts(_, _, _, _)))
        .collect::<Vec<_>>();

    let mut result_array: Vec<(VecPartNotes, VecPartNotes)> = Vec::new();

    for partidx in 0..partscount {
        let mut partnotes_upper: VecPartNotes = Vec::new();
        let mut partnotes_lower: VecPartNotes = Vec::new();

        for sysitem in sysitems.iter() {
            let part = match &sysitem.stype {
                core::sysitem::SysItemType::Parts(partids, _, _, _) => partids.get(partidx),
                _ => continue,
            };
            match part {
                Some(part_id) => {
                    let part = &parts[*part_id];
                    match &part.ptype {
                        PartType::OneVoice(voice) => match &voice.vtype {
                            VoiceType::NoteIds(note_ids, _, _) => {
                                for note_id in note_ids {
                                    partnotes_upper.push((Some(*note_id), sysitem.position, sysitem.id));
                                }
                            }
                            VoiceType::Barpause => {
                                partnotes_upper.push((None, sysitem.position, sysitem.id));
                            }
                        },
                        PartType::TwoVoice(voice_upper, voice_lower) => {
                            match &voice_upper.vtype {
                                VoiceType::NoteIds(note_ids, _, _) => {
                                    for note_id in note_ids {
                                        partnotes_upper.push((Some(*note_id), sysitem.position, sysitem.id));
                                    }
                                }
                                VoiceType::Barpause => {
                                    partnotes_upper.push((None, sysitem.position, sysitem.id));
                                }
                            }
                            match &voice_lower.vtype {
                                VoiceType::NoteIds(note_ids, _, _) => {
                                    for note_id in note_ids {
                                        partnotes_lower.push((Some(*note_id), sysitem.position, sysitem.id));
                                    }
                                }
                                VoiceType::Barpause => {
                                    partnotes_lower.push((None, sysitem.position, sysitem.id));
                                }
                            }
                        }
                        PartType::OtherPart => {}
                    }
                }
                None => {
                    partnotes_upper.push((None, sysitem.position, sysitem.id));
                }
            }
        }

        result_array.push((partnotes_upper.clone(), partnotes_lower.clone()));
    }

    Ok(result_array)
}
