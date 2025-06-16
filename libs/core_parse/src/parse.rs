use core::accidental::Accidental;
use core::barline::BarlineType;
use core::clef::ClefSignature;
use core::complex::{create_complexes_for_part, ComplexInfo};
use core::context::CoreContext;
use core::duration::{NoteDuration, SumDuration};
use core::head::HeadItem;
use core::note::{NoteItem, NoteType};
use core::part::{PartItem, PartType};
use core::stems::headpositions::calculate_head_positions;
use core::stems::stemdirections::calculate_stemitem_directions;
use core::stems::stemitems::create_stem_items_from_notes_in_voice;
use core::sysitem::{SysItem, SysItemType};
use core::voice::{VoiceItem, VoiceType};
use core::ItemId;
use std::cmp::max;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::vec;

pub fn parse_head(_cx: &CoreContext, value: &str) -> Result<HeadItem, Box<dyn Error>> {
    let value = value.trim();
    let level: i8 = value.chars().filter(|c| c.is_numeric() || *c == '-').collect::<String>().parse()?;

    let accidental: Accidental = Accidental::find(value);

    let id = _cx.heads.borrow().len();
    let info: HeadItem = HeadItem {
        id,
        level: level,
        accidental: accidental,
        head_position: None,
    };
    _cx.heads.borrow_mut().push(info.clone());
    Ok(info)
}

pub fn parse_heads(cx: &CoreContext, value: &str) -> Result<Vec<HeadItem>, Box<dyn Error>> {
    let mut str_and_level = value.split(',').map(|s| (s.trim(), level_from_str(s))).collect::<Vec<(&str, i8)>>();
    str_and_level.sort_by_key(|item| item.1); // sort by level

    let head_items = str_and_level.iter().map(|item| item.0).map(|s| parse_head(cx, s)).collect::<Result<Vec<HeadItem>, Box<dyn Error>>>()?;

    Ok(head_items)
}

pub fn parse_notetype(_cx: &CoreContext, value: &str) -> Result<NoteType, Box<dyn Error>> {
    let value = value.trim();
    let ntype = match value {
        "r" => NoteType::Rest,
        _ => {
            let head_infos = parse_heads(_cx, value)?;
            NoteType::Heads(head_infos)
        }
    };
    Ok(ntype)
}

pub fn parse_note(cx: &CoreContext, value: &str, position: usize, duration: NoteDuration) -> Result<usize, Box<dyn Error>> {
    let value = value.trim();
    let ntype = parse_notetype(cx, value)?;
    let id = cx.notes.borrow().len();
    let info: NoteItem = NoteItem { id, position, duration, ntype };
    cx.notes.borrow_mut().push(info);

    Ok(id)
}

pub fn parse_notes(cx: &CoreContext, value: &str) -> Result<(Vec<usize>, SumDuration), Box<dyn Error>> {
    let mut sum_duration: SumDuration = 0;
    let mut duration: NoteDuration = NoteDuration::D4;
    let mut ids: Vec<usize> = Vec::new();
    value.split(" ").filter(|s| !s.is_empty()).for_each(|s| {
        if s.starts_with("D") || s.starts_with("d") {
            duration = NoteDuration::parse(s).unwrap();
        } else {
            let id = parse_note(cx, s, sum_duration, duration.clone()).unwrap();
            sum_duration += duration.clone() as usize;
            ids.push(id);
        }
    });

    Ok((ids, sum_duration))
}

pub fn parse_voicetype(cx: &CoreContext, value: &str) -> Result<VoiceType, Box<dyn Error>> {
    let value = value.trim();
    let vtype = if value.starts_with("bp") {
        VoiceType::Barpause
    } else {
        let (note_ids, sum_duration) = parse_notes(cx, value).expect("Could not parse notes");
        let pattern_values = vec![NoteDuration::D4];
        let stemitem_ids = create_stem_items_from_notes_in_voice(cx, &note_ids, sum_duration, pattern_values).unwrap();

        VoiceType::NoteIds(note_ids, sum_duration, stemitem_ids)
    };
    Ok(vtype)
}

pub fn parse_voice(cx: &CoreContext, value: &str) -> Result<VoiceItem, Box<dyn Error>> {
    let vtype = parse_voicetype(cx, value)?;
    let duration: usize = match &vtype {
        VoiceType::Barpause => 0,
        VoiceType::NoteIds(_, sum_duration, _) => *sum_duration,
    };
    let item = VoiceItem { vtype, duration };
    Ok(item)
}

pub fn parse_parttype(cx: &CoreContext, value: &str) -> Result<PartType, Box<dyn Error>> {
    let value = value.trim();

    let ptype: PartType = if value.starts_with("other-part") {
        PartType::OtherPart
    } else {
        //
        let nr_of_voices = value.split("%").count();
        match nr_of_voices {
            1 => {
                let voiceitem = parse_voice(cx, value)?;
                PartType::OneVoice(voiceitem)
            }
            2 => {
                let values = value.split("%").collect::<Vec<_>>();
                let voiceitem_upper = parse_voice(cx, values[0])?;
                let voiceitem_lower = parse_voice(cx, values[1])?;
                PartType::TwoVoice(voiceitem_upper, voiceitem_lower)
            }
            _ => todo!("Handle more than 2 voices"),
        }
    };
    Ok(ptype)
}

pub fn parse_part(cx: &CoreContext, value: &str) -> Result<ItemId, Box<dyn Error>> {
    let mut value = value.trim();
    if value.starts_with("%") {
        value = value[1..].trim();
    }

    let ptype = parse_parttype(cx, value)?;
    let duration = match &ptype {
        PartType::OneVoice(info) => info.duration,
        PartType::TwoVoice(info_upper, info_lower) => max(info_upper.duration, info_lower.duration),
        PartType::OtherPart => 0,
    };

    let id = cx.parts.borrow().len();

    calculate_stemitem_directions(cx, &ptype);
    let complexids = create_complexes_for_part(cx, &ptype, id);
    // let _ = calculate_head_positions(cx);
    let info = PartItem { id, duration, ptype, complexids };
    cx.parts.borrow_mut().push(info);

    Ok(id)
}

pub fn parse_parts(cx: &CoreContext, value: &str) -> Result<Vec<ItemId>, Box<dyn Error>> {
    let mut value = value.trim();
    if value.starts_with("/") {
        value = value[1..].trim();
    }

    let segments = value.split("/").collect::<Vec<_>>();

    let ids: Vec<ItemId> = segments
        .iter()
        .map(|s| {
            let s = s.trim();
            let id = parse_part(cx, s).expect("Could not parse parts");
            id
        })
        .collect::<Vec<_>>();

    Ok(ids)
}

pub fn parse_sysitemtype(_cx: &CoreContext, value: &str) -> Result<(SysItemType, usize), Box<dyn Error>> {
    let mut value = value.trim();
    if value.starts_with("|") {
        value = value[1..].trim();
    }
    let (t, parts_count) = if value.starts_with("clef") {
        let segments = value.split(" ").filter(|s| !s.is_empty()).skip(1).map(|s| ClefSignature::find(s)).collect::<Vec<_>>();
        let parts_count = segments.len();
        (SysItemType::Clefs(segments), parts_count)
    } else if value.starts_with("bl") {
        (SysItemType::Barline(BarlineType::Single), 1 as usize)
    } else {
        let parts_ids = parse_parts(_cx, value)?;
        let parts_complexes_infos = parts_ids.iter().map(|part_id| get_complex_infos_for_part(_cx, *part_id).unwrap()).collect::<Vec<_>>();
        let max_duration = parts_complexes_infos.iter().map(|pci| pci.last().unwrap()).map(|pci| pci.2 + pci.1).max().unwrap();
        let mut sysitem_positions: BTreeSet<usize> = parts_complexes_infos.iter().flat_map(|pci| pci.iter().map(|c| c.1)).collect();
        sysitem_positions.insert(max_duration);
        let sysitem_positions: Vec<usize> = sysitem_positions.into_iter().collect();
        let sysitem_durations = sysitem_positions.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

        // create a vector of BTreeMaps, where each map contains the complex info for each part at each position
        let parts_complex_pos_map: Vec<BTreeMap<usize, ComplexInfo>> = parts_complexes_infos.iter().map(|pci| pci.iter().cloned().map(|c| (c.1, c)).collect()).collect();

        // create a BTreeMap from sysitem_positions and sysitem_durations
        let positions_durations: BTreeMap<usize, usize> = sysitem_positions.iter().zip(sysitem_durations.iter()).map(|(pos, dur)| (*pos, *dur)).collect();
        let parts_count = parts_ids.len();
        (SysItemType::Parts(parts_ids, max_duration, parts_complex_pos_map, positions_durations), parts_count)
    };

    Ok((t, parts_count))
}

fn get_complex_infos_for_part(cx: &CoreContext, part_id: usize) -> Result<Vec<ComplexInfo>, Box<dyn Error>> {
    let parts = cx.parts.borrow();
    let part = &parts[part_id];
    let complexids = &part.complexids;
    let complexes = cx.complexes.borrow();
    let complex_infos: Vec<ComplexInfo> = complexids
        .iter()
        .map(|&complex_id| {
            let complex = &complexes[complex_id];
            (complex_id, complex.position, complex.duration)
        })
        .collect();

    Ok(complex_infos)
}

pub fn parse_sysitems(cx: &CoreContext, value: &str) -> Result<Vec<ItemId>, Box<dyn Error>> {
    let mut value = value.trim();
    if value.starts_with("|") {
        value = value[1..].trim();
    }
    if value.ends_with("|") {
        value = &value[..value.len() - 1];
    }

    let segments = value.split("|").collect::<Vec<_>>();

    let ids = segments
        .iter()
        .map(|s| {
            let s = s.trim();
            let id = cx.sysitems.borrow().len();
            let (stype, parts_count) = parse_sysitemtype(cx, s).expect("Could not parse sysitemtype");
            let s = SysItem {
                id,
                stype,
                parts_count, // complexes_durations: vec![],
            };
            cx.sysitems.borrow_mut().push(s);
            id
        })
        .collect::<Vec<_>>();
    Ok(ids)
}

pub fn level_from_str(s: &str) -> i8 {
    let level: i8 = s.chars().filter(|c| c.is_numeric() || *c == '-').collect::<String>().parse().unwrap();
    level
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_n() {
        let cx = CoreContext::new();
        let _ = parse_note(&cx, "b1,-3", 0, NoteDuration::D8).unwrap();
        dbg!(&cx);
    }

    #[test]
    fn test_ns() {
        let cx = CoreContext::new();
        let ids = parse_notes(cx, "1,2 D8 2,-5").unwrap();
        dbg!(&ids);
        dbg!(&cx);
    }

    #[test]
    fn test_v() {
        let cx = CoreContext::new();
        let _ = parse_voice(cx, "1 2 3").unwrap();
        dbg!(&cx);
    }

    #[test]
    fn test_p() {
        let cx = CoreContext::new();
        let _ = parse_part(cx, "11 % 22").unwrap();
        dbg!(&cx);
    }

    #[test]
    fn test_ps() {
        let cx = CoreContext::new();
        // let _ = parse_parts(cx, "0 1 / 1 % 0 d8 0 1 d16 2").unwrap();
        let _ = parse_parts(cx, "0 D2 1 / 0 1 D8 2 % 0 D16 1 2").unwrap();
        // let _ = parse_parts(cx, "0 1 D8 2 % 0 D16 1 2").unwrap();
        // let _ = parse_parts(cx, "d4 0 0 % d8 1  ").unwrap();
        dbg!(&cx);
    }

    #[test]
    fn test_s() {
        let cx = CoreContext::new();
        // let _ = parse_sysitemtype(cx, "clef G F").unwrap();
        let _ = parse_sysitemtype(cx, "0 D2 1 / 0 1 D8 2 % 0 D16 1 2").unwrap();
        // dbg!(&cx);
    }

    #[test]
    fn test_maj03() {
        let cx = CoreContext::new();
        let _ = parse_sysitems(cx, "|clef G | D4. -2,-3 D8 -4 % D16 2 3 4 5 D8 3 4 / D2. 0  |bl | 0 / 1").unwrap();
        // let _ = parse_sysitems(cx, "0 % 0").unwrap();
        // let _check = check_sysitems_parts_integrity(cx, ids);
        // dbg!(&cx.notes.borrow().len());
        // assert!(cx.notes.borrow().len() == 10, "Expected 10 notes, found {}", cx.notes.borrow().len());
        // dbg!(&cx.complexes);
        // dbg!(&cx.stemitems);
        // dbg!(&cx.parts);
    }
}
