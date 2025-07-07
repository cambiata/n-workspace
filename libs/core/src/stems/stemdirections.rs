use crate::{
    context::CoreContext,
    direction::{DirectionUAD, DirectionUD},
    part::PartType,
    voice::VoiceType,
};

use super::stemitems::StemType;

pub struct StemDirectionUtils;
impl StemDirectionUtils {
    pub fn set_direction_auto(cx: &CoreContext, stemitem_id: usize) {
        let mut cx_stemitems = cx.stemitems.borrow_mut();
        let stemitem = cx_stemitems.get_mut(stemitem_id).unwrap();
        match stemitem.stype {
            StemType::NoteWithStem(ref mut item) => {
                let direction = DirectionUAD::from_level(item.bottom_level + item.top_level);
                stemitem.direction = Some(direction.clone());
                cx.map_noteid_direction.borrow_mut().insert(item.note.id, direction.clone());
            }
            StemType::NotesBeamed(ref mut items) => {
                let top_level = items.iter().map(|item| item.top_level).min().unwrap();
                let bottom_level = items.iter().map(|item| item.bottom_level).max().unwrap();
                let direction = DirectionUAD::from_level(bottom_level + top_level);
                stemitem.direction = Some(direction.clone());
                for item in items.iter_mut() {
                    cx.map_noteid_direction.borrow_mut().insert(item.note.id, direction.clone());
                }
            }
            _ => {}
        }
    }

    pub fn set_direction_force(cx: &CoreContext, stemitem_id: usize, force_direction: DirectionUD) {
        let mut cx_stemitems = cx.stemitems.borrow_mut();
        let stemitem = cx_stemitems.get_mut(stemitem_id).unwrap();
        stemitem.direction = Some(force_direction.clone());

        match stemitem.stype {
            StemType::NoteWithStem(ref mut item) => {
                cx.map_noteid_direction.borrow_mut().insert(item.note.id, force_direction.clone());
            }
            StemType::NotesBeamed(ref mut items) => {
                for item in items.iter_mut() {
                    cx.map_noteid_direction.borrow_mut().insert(item.note.id, force_direction.clone());
                }
            }
            _ => {}
        }
    }
}

pub fn calculate_stemitem_directions(cx: &CoreContext, ptype: &PartType) -> Result<(), Box<dyn std::error::Error>> {
    let mut stemitems = cx.stemitems.borrow_mut();

    match ptype {
        PartType::OneVoice(ref voice_item) => match &voice_item.vtype {
            VoiceType::NoteIds(_, _, stemitem_ids) => {
                for stemitem_id in stemitem_ids {
                    let stemitem = stemitems.get_mut(*stemitem_id).unwrap();
                    match stemitem.stype {
                        StemType::NoteWithStem(ref item) => {
                            let direction = DirectionUAD::from_level(item.bottom_level + item.top_level);
                            stemitem.direction = Some(direction);
                        }
                        StemType::NotesBeamed(ref items) => {
                            let top_level = items.iter().map(|item| item.top_level).min().unwrap();
                            let bottom_level = items.iter().map(|item| item.bottom_level).max().unwrap();
                            let direction = DirectionUAD::from_level(bottom_level + top_level);
                            stemitem.direction = Some(direction);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        },
        PartType::TwoVoice(ref voice_item_upper, ref voice_item_lower) => match (&voice_item_upper.vtype, &voice_item_lower.vtype) {
            (VoiceType::NoteIds(_, duration_upper, stemitem_ids_upper), VoiceType::NoteIds(_, duration_lower, stemitem_ids_lower)) => {
                let duration_shorter = *duration_upper.min(duration_lower);
                for stemitem_id in stemitem_ids_upper {
                    let stemitem = stemitems.get_mut(*stemitem_id).unwrap();
                    if stemitem.position >= duration_shorter {
                        match stemitem.stype {
                            StemType::NoteWithStem(ref item) => {
                                let direction = DirectionUAD::from_level(item.bottom_level + item.top_level);
                                stemitem.direction = Some(direction);
                            }
                            StemType::NotesBeamed(ref items) => {
                                let top_level = items.iter().map(|item| item.top_level).min().unwrap();
                                let bottom_level = items.iter().map(|item| item.bottom_level).max().unwrap();
                                let direction = DirectionUAD::from_level(bottom_level + top_level);
                                stemitem.direction = Some(direction);
                            }
                            _ => {}
                        }
                    } else {
                        stemitem.direction = Some(DirectionUD::Up);
                    }
                }

                for stemitem_id in stemitem_ids_lower {
                    let stemitem = stemitems.get_mut(*stemitem_id).unwrap();
                    if stemitem.position >= duration_shorter {
                        match stemitem.stype {
                            StemType::NoteWithStem(ref item) => {
                                let direction = DirectionUAD::from_level(item.bottom_level + item.top_level);
                                stemitem.direction = Some(direction);
                            }
                            StemType::NotesBeamed(ref items) => {
                                let top_level = items.iter().map(|item| item.top_level).min().unwrap();
                                let bottom_level = items.iter().map(|item| item.bottom_level).max().unwrap();
                                let direction = DirectionUAD::from_level(bottom_level + top_level);
                                stemitem.direction = Some(direction);
                            }
                            _ => {}
                        }
                    } else {
                        stemitem.direction = Some(DirectionUD::Down);
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
