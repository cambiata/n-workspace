use core::{
    barline::BarlineType,
    clef::ClefSignature,
    complex::ComplexUtils,
    context::CoreContext,
    direction::DirectionUD,
    hpart::{HPartAttributes, HPartItem, HPartItemsColumn, HPartItemsColumnType, HPartItemsRow, HPartMusicType, HPartType, VoiceType2},
    key::KeySignature,
    stems::stemdirections::StemDirectionUtils,
    sysitem::SysItemTypeId,
    time::{TimeDenominator, TimeNominator, TimeSignature},
};
use std::collections::HashMap;

use crate::parse2::Parse2;

pub struct Parse2Utils;

#[allow(unused_mut)]
impl Parse2Utils {
    pub fn create_columns_of_parts2(_cx: &CoreContext, bpvmap: &mut [SysItemTypeId], parts_count: usize) -> Result<(), Box<dyn std::error::Error>> {
        let mut clef_map: HashMap<usize, ClefSignature> = HashMap::new();
        let mut time_map: HashMap<usize, TimeSignature> = HashMap::new();
        let mut key_map: HashMap<usize, KeySignature> = HashMap::new();

        let mut column_position = 0;
        for (col_idx, item) in bpvmap.iter().enumerate() {
            let mut column_duration: usize = 0;

            match item {
                SysItemTypeId::Clefs(segments) => {
                    let mut items_ids: Vec<usize> = vec![];
                    for part_idx in 0..parts_count {
                        let item = &segments[part_idx];
                        let clef_signature = ClefSignature::find(item);
                        clef_map.insert(part_idx, clef_signature.clone());
                        let hpart: HPartType = HPartType::Clef(clef_signature);
                        let id = _cx.hparts.borrow().len();
                        let item: HPartItem = HPartItem {
                            id, // This will be set later
                            parttype: hpart,
                            position: column_position,
                            duration: column_duration,
                            part_idx,
                            col_idx,
                        };
                        _cx.hparts.borrow_mut().push(item);
                        items_ids.push(id);
                    }
                    let id = _cx.columns.borrow().len();
                    let column: HPartItemsColumn = HPartItemsColumn {
                        id,
                        hptype: HPartItemsColumnType::Clefs(items_ids),
                        position: column_position,
                        duration: column_duration,
                        col_idx,
                    };
                    _cx.columns.borrow_mut().push(column);
                }
                SysItemTypeId::Barlines(segments) => {
                    let mut items_ids: Vec<usize> = vec![];
                    for part_idx in 0..parts_count {
                        // let item = &segments[part_idx];
                        let barline = BarlineType::Single;
                        let hpart: HPartType = HPartType::Barline(barline);
                        let id = _cx.hparts.borrow().len();
                        let item: HPartItem = HPartItem {
                            id,
                            parttype: hpart,
                            position: column_position,
                            duration: column_duration,
                            part_idx,
                            col_idx,
                        };
                        _cx.hparts.borrow_mut().push(item);
                        items_ids.push(id);
                    }
                    let id = _cx.columns.borrow().len();
                    let column: HPartItemsColumn = HPartItemsColumn {
                        id,
                        hptype: HPartItemsColumnType::Barlines(items_ids),
                        position: column_position,
                        duration: column_duration,
                        col_idx,
                    };
                    _cx.columns.borrow_mut().push(column);
                }

                SysItemTypeId::Parts(parts) => {
                    let mut column_duration: usize = 0;

                    let mut item_types: Vec<HPartType> = vec![];

                    for part_idx in 0..parts_count {
                        let item = &parts[part_idx];

                        let attr: HPartAttributes = HPartAttributes {
                            clef: clef_map.get(&part_idx).cloned().unwrap_or(ClefSignature::Treble),
                            time: time_map.get(&part_idx).cloned().unwrap_or(TimeSignature::TimeSignature(TimeNominator::Four, TimeDenominator::Four)),
                            key: key_map.get(&part_idx).cloned().unwrap_or(KeySignature::Neutral),
                        };

                        let default_duration = TimeSignature::get_duration(&attr.time);
                        let htype = match item.len() {
                            1 => {
                                let voicetype = Parse2::voicetype(_cx, &item[0], default_duration)?;
                                let complexes = match &voicetype {
                                    VoiceType2::NoteIds { note_ids, duration, stemitem_ids: _ } => {
                                        column_duration = column_duration.max(*duration);
                                        ComplexUtils::create_complexes_for_one_voice(_cx, &note_ids, *duration, true, part_idx, column_position)
                                    }
                                    _ => Vec::new(),
                                };

                                HPartType::Music(HPartMusicType::OneVoice { voice: voicetype }, complexes, attr)
                            }
                            2 => {
                                let upper = Parse2::voicetype(_cx, &item[0], default_duration)?;
                                let lower = Parse2::voicetype(_cx, &item[1], default_duration)?;
                                let complexes = match (&upper, &lower) {
                                    (
                                        VoiceType2::NoteIds {
                                            note_ids: upper_ids,
                                            duration: upper_dur,
                                            stemitem_ids: _,
                                        },
                                        VoiceType2::NoteIds {
                                            note_ids: lower_ids,
                                            duration: lower_dur,
                                            stemitem_ids: _,
                                        },
                                    ) => {
                                        column_duration = column_duration.max(*upper_dur).max(*lower_dur);
                                        ComplexUtils::create_complexes_for_two_voices(_cx, &upper_ids, &lower_ids, *upper_dur, *lower_dur, column_position)
                                    }

                                    (_, VoiceType2::NoteIds { note_ids, duration, stemitem_ids: _ }) => {
                                        column_duration = column_duration.max(*duration);
                                        ComplexUtils::create_complexes_for_one_voice(_cx, &note_ids, *duration, false, part_idx, column_position)
                                    }
                                    (VoiceType2::NoteIds { note_ids, duration, stemitem_ids: _ }, _) => {
                                        column_duration = column_duration.max(*duration);
                                        ComplexUtils::create_complexes_for_one_voice(_cx, &note_ids, *duration, true, part_idx, column_position)
                                    }
                                    _ => Vec::new(),
                                };
                                HPartType::Music(HPartMusicType::TwoVoices { upper, lower }, complexes, attr)
                            }
                            _ => {
                                panic!("Should not happen");
                            }
                        };

                        item_types.push(htype);
                    }

                    let mut item_ids: Vec<usize> = vec![];
                    for (part_idx, htype) in item_types.into_iter().enumerate() {
                        let id = _cx.hparts.borrow().len();
                        let item: HPartItem = HPartItem {
                            id, // This will be set later
                            parttype: htype,
                            position: column_position,
                            duration: column_duration,
                            part_idx,
                            col_idx,
                        };
                        _cx.hparts.borrow_mut().push(item);
                        item_ids.push(id);
                    }

                    let id = _cx.columns.borrow().len();
                    let column: HPartItemsColumn = HPartItemsColumn {
                        id,
                        hptype: HPartItemsColumnType::Musics(item_ids),
                        position: column_position,
                        duration: column_duration,
                        col_idx,
                    };
                    _cx.columns.borrow_mut().push(column);
                    column_position += column_duration;
                }
                _ => {}
            }
        }

        dbg!(&_cx.columns.borrow());

        //------------------------------------------

        for part_idx in 0..parts_count {
            let mut ids: Vec<usize> = Vec::new();
            for column in _cx.columns.borrow().iter() {
                let id = match &column.hptype {
                    HPartItemsColumnType::Musics(ids) | HPartItemsColumnType::Barlines(ids) | HPartItemsColumnType::Clefs(ids) => ids[part_idx],
                };
                ids.push(id);
            }
            let id = _cx.rows.borrow().len();
            let row: HPartItemsRow = HPartItemsRow { id, hpart_ids: ids, part_idx };
            _cx.rows.borrow_mut().push(row);
        }

        dbg!(&_cx.rows.borrow());

        Ok(())
    }

    pub fn split_double_voices(bpvmap: &mut [SysItemTypeId], parts_count: usize) -> usize {
        // todo!()
        let mut max_voices_count = 0;
        for part_idx in 0..parts_count {
            let mut part_voices_count = 0;
            for item in bpvmap.iter() {
                match item {
                    SysItemTypeId::Parts(parts) => {
                        if let Some(part) = parts.get(part_idx) {
                            part_voices_count = part_voices_count.max(part.len());
                        }
                    }
                    _ => {}
                }
            }

            println!("Max voices for part {}: {}", part_idx, part_voices_count);

            if part_voices_count > 1 {
                for item in bpvmap.iter_mut() {
                    match item {
                        SysItemTypeId::Parts(parts) => {
                            if let Some(part) = parts.get_mut(part_idx) {
                                if part.len() != part_voices_count {
                                    println!("Fix voices for part {}: {:?}", part_idx, part);
                                    parts.insert(part_idx + 1, vec!["Voice-added3".to_string()]);
                                // Default voice
                                } else {
                                    println!("Part {} already has correct voices - Split them!: {:?}", part_idx, part);
                                    let v = part.pop();
                                    // let v = part.pop();
                                    if let Some(voice) = v {
                                        println!("HOHO - Split voice: {}", voice);
                                        parts.insert(part_idx + 1, vec![voice.clone()]);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            max_voices_count += part_voices_count;
        }
        max_voices_count
    }

    pub fn get_parts_config(bpvmap: &[SysItemTypeId]) -> Vec<usize> {
        let mut parts_config: Vec<usize> = vec![];

        let mut max_parts_count = 0;
        for item in bpvmap.iter() {
            let part_count = match item {
                SysItemTypeId::Clefs(x) | SysItemTypeId::Barlines(x) => x.len(),
                SysItemTypeId::Parts(x) => x.len(),
            };
            max_parts_count = max_parts_count.max(part_count);
        }

        for part_idx in 0..max_parts_count {
            let mut voices_count = 1;
            for item in bpvmap.iter() {
                match item {
                    SysItemTypeId::Parts(parts) => {
                        if let Some(part) = parts.get(part_idx) {
                            voices_count = voices_count.max(part.len());
                        }
                    }
                    _ => {}
                }
            }
            parts_config.push(voices_count);
        }

        parts_config
    }

    pub fn correct_to_parts_count(bpvmap: &mut [SysItemTypeId], parts_count: usize) {
        // correct clefs and barlines
        for item in bpvmap.iter_mut() {
            //
            match item {
                SysItemTypeId::Clefs(ref mut clef_segments) => {
                    // dbg!(&clef_segments);
                    while *&clef_segments.len() < parts_count {
                        clef_segments.push("G-added".to_string()); // Default clef
                    }
                    // dbg!(&clef_segments);
                }
                SysItemTypeId::Barlines(ref mut barline_segments) => {
                    while *&barline_segments.len() < parts_count {
                        barline_segments.push("Single-added".to_string()); // Default barline
                    }
                }
                _ => {}
            }
        }

        // correct parts and voices
        for item_idx in 0..parts_count {
            let mut max_voices_count = 0;
            // println!("Check part:{}", item_idx);
            for item in bpvmap.iter() {
                match item {
                    SysItemTypeId::Parts(parts) => {
                        let part = &parts.get(item_idx);
                        match part {
                            Some(part) => {
                                max_voices_count = max_voices_count.max(part.len());
                            }
                            None => {}
                        }
                    }
                    _ => {}
                }
            }
            // println!("Max voices for part {}: {}", item_idx, max_voices_count);

            for item in bpvmap.iter_mut() {
                match item {
                    SysItemTypeId::Parts(ref mut parts) => {
                        let part = &mut parts.get_mut(item_idx);
                        // dbg!(part);
                        match part {
                            Some(part) => {
                                while *&part.len() < max_voices_count {
                                    part.push("Voice-added".to_string()); // Default voice
                                }
                            }
                            None => {
                                parts.push(vec!["Voice-added".to_string()]);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub(crate) fn set_stemitems_directions(cx: &CoreContext) {
        let hparts = cx.hparts.borrow();
        let rows = cx.rows.borrow();
        for row in rows.iter() {
            for id in row.hpart_ids.iter() {
                let item = hparts.get(*id).unwrap();

                match &item.parttype {
                    HPartType::Music(HPartMusicType::OneVoice { voice }, complexes, _) => match voice {
                        VoiceType2::NoteIds {
                            note_ids: _,
                            duration: _,
                            stemitem_ids,
                        } => {
                            stemitem_ids.iter().for_each(|stemitem_id| {
                                StemDirectionUtils::set_direction_auto(cx, *stemitem_id);
                            });
                        }
                        _ => {}
                    },
                    HPartType::Music(HPartMusicType::TwoVoices { upper, lower }, complexes, _) => {
                        match upper {
                            VoiceType2::NoteIds {
                                note_ids: _,
                                duration: _,
                                stemitem_ids,
                            } => {
                                stemitem_ids.iter().for_each(|stemitem_id| {
                                    StemDirectionUtils::set_direction_force(cx, *stemitem_id, DirectionUD::Up);
                                });
                            }
                            _ => {}
                        }
                        match lower {
                            VoiceType2::NoteIds {
                                note_ids: _,
                                duration: _,
                                stemitem_ids,
                            } => {
                                stemitem_ids.iter().for_each(|stemitem_id| {
                                    StemDirectionUtils::set_direction_force(cx, *stemitem_id, DirectionUD::Down);
                                });
                            }
                            _ => {}
                        }
                    }

                    _ => {
                        dbg!(&item);
                    }
                }
            }
        }
    }

    /*
    pub(crate) fn set_head_positions(cx: &CoreContext) {
        let hparts = cx.hparts.borrow();
        let rows = cx.rows.borrow();
        for row in rows.iter() {
            for id in row.hpart_ids.iter() {
                let item = hparts.get(*id).unwrap();
                dbg!(&item);
                match &item.parttype {
                    HPartType::Music(HPartMusicType::OneVoice { voice, complexes: _ }, _) => match voice {
                        VoiceType2::NoteIds {
                            note_ids: _,
                            duration: _,
                            stemitem_ids,
                        } => {
                            stemitem_ids.iter().for_each(|stemitem_id| {
                                StemDirectionUtils::set_direction_auto(cx, *stemitem_id);
                            });
                        }
                        _ => {}
                    },
                    HPartType::Music(HPartMusicType::TwoVoices { upper, lower, complexes: _ }, _) => {
                        match upper {
                            VoiceType2::NoteIds {
                                note_ids: _,
                                duration: _,
                                stemitem_ids,
                            } => {
                                stemitem_ids.iter().for_each(|stemitem_id| {
                                    StemDirectionUtils::set_direction_force(cx, *stemitem_id, DirectionUD::Up);
                                });
                            }
                            _ => {}
                        }
                        match lower {
                            VoiceType2::NoteIds {
                                note_ids: _,
                                duration: _,
                                stemitem_ids,
                            } => {
                                stemitem_ids.iter().for_each(|stemitem_id| {
                                    StemDirectionUtils::set_direction_force(cx, *stemitem_id, DirectionUD::Down);
                                });
                            }
                            _ => {}
                        }
                    }

                    _ => {
                        dbg!(&item);
                    }
                }
            }
        }
    }
     */
}
