use core::{
    barline::BarlineType,
    clef::ClefSignature,
    complex::ComplexUtils,
    context::CoreContext,
    direction::DirectionUD,
    hpart::{HPartAttributes, HPartItem, HPartItemsColumn, HPartItemsColumnType, HPartItemsRow, HPartMusicType, HPartType, VoiceType2},
    key::KeySignature,
    stems::{headpositions::HeadPositionUtils, stemdirections::StemDirectionUtils},
    sysitem::SysItemTypeId,
    ties::{CheckedTieFrom, CheckedTieTo, TieFrom},
    time::{TimeDenominator, TimeNominator, TimeSignature},
};
use std::{collections::HashMap, error::Error};

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
                            hptype: hpart,
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
                SysItemTypeId::Barlines(_segments) => {
                    let mut items_ids: Vec<usize> = vec![];
                    for part_idx in 0..parts_count {
                        // let item = &segments[part_idx];
                        let barline = BarlineType::Single;
                        let hpart: HPartType = HPartType::Barline(barline);
                        let id = _cx.hparts.borrow().len();
                        let item: HPartItem = HPartItem {
                            id,
                            hptype: hpart,
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

                                HPartType::Music {
                                    mtype: HPartMusicType::OneVoice { voice: voicetype },
                                    complexes,
                                    attr,
                                }
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
                                HPartType::Music {
                                    mtype: HPartMusicType::TwoVoices { upper, lower },
                                    complexes,
                                    attr,
                                }
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
                            hptype: htype,
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
            }
        }

        //------------------------------------------

        // for part_idx in 0..parts_count {
        //     let mut ids: Vec<usize> = Vec::new();
        //     for column in _cx.columns.borrow().iter() {
        //         let id = match &column.hptype {
        //             HPartItemsColumnType::Musics(ids) | HPartItemsColumnType::Barlines(ids) | HPartItemsColumnType::Clefs(ids) => ids[part_idx],
        //         };
        //         ids.push(id);
        //     }
        //     let id = _cx.rows.borrow().len();
        //     let row: HPartItemsRow = HPartItemsRow { id, hpart_ids: ids, part_idx };
        //     _cx.rows.borrow_mut().push(row);
        // }

        // dbg!(&_cx.rows.borrow());

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

    pub fn create_rows_from_columns(cx: &CoreContext) -> Result<(), Box<dyn Error>> {
        let cx_columns = cx.columns.borrow();
        let first_column = cx_columns.first().ok_or("No columns found")?;
        let parts_count = match &first_column.hptype {
            HPartItemsColumnType::Musics(ids) | HPartItemsColumnType::Barlines(ids) | HPartItemsColumnType::Clefs(ids) => ids.len(),
        };

        for part_idx in 0..parts_count {
            let mut ids: Vec<usize> = Vec::new();
            for column in cx_columns.iter() {
                let id = match &column.hptype {
                    HPartItemsColumnType::Musics(ids) | HPartItemsColumnType::Barlines(ids) | HPartItemsColumnType::Clefs(ids) => ids[part_idx],
                };
                ids.push(id);
            }
            let id = cx.rows.borrow().len();
            let row: HPartItemsRow = HPartItemsRow { id, hpart_ids: ids, part_idx };
            cx.rows.borrow_mut().push(row);
        }

        Ok(())
    }

    pub fn set_stemitems_directions(cx: &CoreContext) -> Result<(), Box<dyn Error>> {
        let hparts = cx.hparts.borrow();
        let rows = cx.rows.borrow();
        for row in rows.iter() {
            for id in row.hpart_ids.iter() {
                let item = hparts.get(*id).unwrap();

                match &item.hptype {
                    HPartType::Music {
                        mtype: HPartMusicType::OneVoice { voice },
                        complexes: _,
                        attr: _,
                    } => match voice {
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
                    HPartType::Music {
                        mtype: HPartMusicType::TwoVoices { upper, lower },
                        complexes: _,
                        attr: _,
                    } => {
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
                        // dbg!(&item);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn calculate_head_positions(cx: &CoreContext) -> Result<(), Box<dyn Error>> {
        HeadPositionUtils::set_head_positions(&cx.stemitems.borrow(), &mut cx.map_head_position.borrow_mut());
        Ok(())
    }

    pub fn map_notes_by_voices(cx: &CoreContext) -> Result<(), Box<dyn Error>> {
        let rows = cx.rows.borrow();
        let hparts = cx.hparts.borrow();

        let mut map_notids_per_voice = cx.map_notids_per_voice.borrow_mut();
        let mut map_stemitem_ids_per_voice = cx.map_stemitem_ids_per_voice.borrow_mut();

        for (_part_idx, row) in rows.iter().enumerate() {
            let hpart_ids = &row.hpart_ids;
            for hpart_id in hpart_ids.iter() {
                let hpart = hparts.get(*hpart_id).unwrap();
                match &hpart.hptype {
                    HPartType::Music { mtype, complexes: _, attr: _ } => match mtype {
                        HPartMusicType::OneVoice { voice } => {
                            if let VoiceType2::NoteIds { note_ids, duration: _, stemitem_ids } = voice {
                                map_notids_per_voice.entry((hpart.part_idx, 0)).or_insert_with(Vec::new).extend(note_ids.iter().cloned());
                                map_stemitem_ids_per_voice.entry((hpart.part_idx, 0)).or_insert_with(Vec::new).extend(stemitem_ids.iter().cloned());
                            }
                        }
                        HPartMusicType::TwoVoices { upper, lower } => {
                            if let VoiceType2::NoteIds { note_ids, duration: _, stemitem_ids } = upper {
                                map_notids_per_voice.entry((hpart.part_idx, 0)).or_insert_with(Vec::new).extend(note_ids.iter().cloned());
                                map_stemitem_ids_per_voice.entry((hpart.part_idx, 0)).or_insert_with(Vec::new).extend(stemitem_ids.iter().cloned());
                            }
                            if let VoiceType2::NoteIds { note_ids, duration: _, stemitem_ids } = lower {
                                map_notids_per_voice.entry((hpart.part_idx, 1)).or_insert_with(Vec::new).extend(note_ids.iter().cloned());
                                map_stemitem_ids_per_voice.entry((hpart.part_idx, 1)).or_insert_with(Vec::new).extend(stemitem_ids.iter().cloned());
                            }
                        }
                    },
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn resolve_ties_to(_cx: &CoreContext) -> Result<(), Box<dyn Error>> {
        // let map_notids_per_voice = cx.map_notids_per_voice.borrow();
        // let map_ties_to = cx.map_noteid_tiesto.borrow_mut();
        // let notes = cx.notes.borrow();

        // for (part_idx, voice_idx) in map_notids_per_voice.keys() {
        //     let note_ids = map_notids_per_voice.get(&(*part_idx, *voice_idx)).unwrap();

        //     for note_id in note_ids {
        //         if map_ties_to.contains_key(&note_id) {
        //             dbg!(&note_id);

        //             let levels = map_ties_to.get(note_id).ok_or("Note has no ties to")?;
        //             dbg!(&levels);
        //             for level in levels.iter() {
        //                 match level {
        //                     TieTo::Level(level) => {
        //                         // already has resolved tie to?
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }

        Ok(())
    }

    pub fn resolve_ties_from(cx: &CoreContext) -> Result<(), Box<dyn Error>> {
        let map_notids_per_voice = cx.map_notids_per_voice.borrow();
        let map_ties_from = cx.map_noteid_tiesfrom.borrow_mut();
        let notes = cx.notes.borrow();

        for (part_idx, voice_idx) in map_notids_per_voice.keys() {
            let note_ids = map_notids_per_voice.get(&(*part_idx, *voice_idx)).unwrap();

            for n in note_ids.windows(2) {
                let left_id = n[0];
                let right_id = n[1];
                if map_ties_from.contains_key(&left_id) {
                    // Note has ties from
                    let left_tied_levels = map_ties_from.get(&left_id).ok_or("Left note has no ties from")?;

                    let right_note = notes.get(right_id).ok_or("Right note not found")?;
                    if let Some(right_head_levels) = right_note.get_head_levels() {
                        // dbg!(&left_tied_levels, &right_head_levels);
                        for left_tied_level in left_tied_levels.iter() {
                            match left_tied_level {
                                TieFrom::Level(left_level) => {
                                    if right_head_levels.contains(left_level) {
                                        // Found a tie resolution
                                        let tie_to = CheckedTieTo::Resolved(*left_level);
                                        cx.map_noteid_resolvedtiesto.borrow_mut().entry(left_id).or_insert_with(Vec::new).push(tie_to);
                                        let tie_from = CheckedTieFrom::Resolved(*left_level);
                                        cx.map_noteid_resolvedtiesfrom.borrow_mut().entry(right_id).or_insert_with(Vec::new).push(tie_from);
                                    } else {
                                        // Did not find a tie resolution
                                        let tie_to = CheckedTieTo::Unresolved(*left_level);
                                        cx.map_noteid_resolvedtiesto.borrow_mut().entry(left_id).or_insert_with(Vec::new).push(tie_to);
                                    }
                                } // _ => {
                                  //     todo!("Handle other TieFrom types");
                                  // }
                            }
                        }
                    }
                }
            }

            // Handle very last note in the voice
            if let Some(last_id) = note_ids.last() {
                if map_ties_from.contains_key(last_id) {
                    // Note has ties from
                    let left_tied_levels = map_ties_from.get(last_id).ok_or("Last note has no ties from")?;
                    for left_tied_level in left_tied_levels.iter() {
                        match left_tied_level {
                            TieFrom::Level(left_level) => {
                                let tie_to = CheckedTieTo::Unresolved(*left_level);
                                cx.map_noteid_resolvedtiesto.borrow_mut().entry(*last_id).or_insert_with(Vec::new).push(tie_to);
                            } // _ => {
                              //     todo!("Handle other TieFrom types");
                              // }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
