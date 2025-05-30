use core::accidental::Accidental;
use core::clef::ClefSignature;
use core::context::CoreContext;
use core::duration::{Duration, SumDuration};
use core::head::HeadItem;
use core::note::{NoteItem, NoteType};
use core::part::{complex::create_complexes_for_part, PartItem, PartType};
use core::sysitem::{SysItem, SysItemType};
use core::voice::stemdirections::calculate_stemitem_directions;
use core::voice::stemitems::create_stem_items_from_notes_in_voice;
use core::voice::{VoiceItem, VoiceType};
use core::ItemId;
use std::cmp::max;

pub fn parse_head(_cx: &CoreContext, value: &str) -> Result<HeadItem, Box<dyn std::error::Error>> {
    let value = value.trim();
    let level: i8 = value.chars().filter(|c| c.is_numeric() || *c == '-').collect::<String>().parse()?;

    let accidental: Accidental = Accidental::find(value);
    let info: HeadItem = HeadItem { level: level, accidental: accidental };
    Ok(info)
}

pub fn parse_heads(cx: &CoreContext, value: &str) -> Result<Vec<HeadItem>, Box<dyn std::error::Error>> {
    let mut s_and_level = value.split(',').map(|s| (s.trim(), level_from_str(s))).collect::<Vec<(&str, i8)>>();
    s_and_level.sort_by_key(|item| item.1); // sort by level

    let infos = s_and_level
        .iter()
        .map(|item| item.0)
        .map(|s| parse_head(cx, s))
        .collect::<Result<Vec<HeadItem>, Box<dyn std::error::Error>>>()?;
    Ok(infos)
}

pub fn parse_notetype(_cx: &CoreContext, value: &str) -> Result<NoteType, Box<dyn std::error::Error>> {
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

pub fn parse_note(cx: &CoreContext, value: &str, position: usize, duration: Duration) -> Result<usize, Box<dyn std::error::Error>> {
    let value = value.trim();
    let ntype = parse_notetype(cx, value)?;
    let id = cx.notes.borrow().len();
    let info: NoteItem = NoteItem { id, position, duration, ntype };
    cx.notes.borrow_mut().push(info);

    Ok(id)
}

pub fn parse_notes(cx: &CoreContext, value: &str) -> Result<(Vec<usize>, SumDuration), Box<dyn std::error::Error>> {
    let mut sum_duration: SumDuration = 0;
    let mut duration: Duration = Duration::D4;
    let mut ids: Vec<usize> = Vec::new();
    value.split(" ").filter(|s| !s.is_empty()).for_each(|s| {
        if s.starts_with("D") || s.starts_with("d") {
            duration = Duration::parse(s).unwrap();
        } else {
            let id = parse_note(cx, s, sum_duration, duration.clone()).unwrap();
            sum_duration += duration.clone() as usize;
            ids.push(id);
        }
    });

    Ok((ids, sum_duration))
}

pub fn parse_voicetype(cx: &CoreContext, value: &str) -> Result<VoiceType, Box<dyn std::error::Error>> {
    let value = value.trim();
    let vtype = if value.starts_with("bp") {
        VoiceType::Barpause
    } else {
        let (note_ids, sum_duration) = parse_notes(cx, value).expect("Could not parse notes");
        let pattern_values = vec![Duration::D4];
        let stemitem_ids = create_stem_items_from_notes_in_voice(cx, &note_ids, sum_duration, pattern_values).unwrap();

        VoiceType::NoteIds(note_ids, sum_duration, stemitem_ids)
    };
    Ok(vtype)
}

pub fn parse_voice(cx: &CoreContext, value: &str) -> Result<VoiceItem, Box<dyn std::error::Error>> {
    let vtype = parse_voicetype(cx, value)?;
    let duration: usize = match &vtype {
        VoiceType::Barpause => 0,
        VoiceType::NoteIds(_, sum_duration, _) => *sum_duration,
    };
    let item = VoiceItem { vtype, duration };
    Ok(item)
}

pub fn parse_parttype(cx: &CoreContext, value: &str) -> Result<PartType, Box<dyn std::error::Error>> {
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

pub fn parse_part(cx: &CoreContext, value: &str) -> Result<(ItemId, SumDuration), Box<dyn std::error::Error>> {
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
    create_complexes_for_part(cx, &ptype, id);

    let info = PartItem { id, duration, ptype };
    cx.parts.borrow_mut().push(info);

    Ok((id, duration))
}

pub fn parse_parts(cx: &CoreContext, value: &str) -> Result<Vec<(ItemId, SumDuration)>, Box<dyn std::error::Error>> {
    let mut value = value.trim();
    if value.starts_with("/") {
        value = value[1..].trim();
    }

    let segments = value.split("/").collect::<Vec<_>>();

    let ids_and_durations: Vec<(ItemId, SumDuration)> = segments
        .iter()
        .map(|s| {
            let s = s.trim();
            let id_and_duration = parse_part(cx, s).expect("Could not parse parts");
            id_and_duration
        })
        .collect::<Vec<_>>();

    Ok(ids_and_durations)
}

pub fn parse_sysitemtype(_cx: &CoreContext, value: &str) -> Result<SysItemType, Box<dyn std::error::Error>> {
    let mut value = value.trim();
    if value.starts_with("|") {
        value = value[1..].trim();
    }

    let t = if value.starts_with("clef") {
        let segments = value.split(" ").filter(|s| !s.is_empty()).skip(1).map(|s| ClefSignature::find(s)).collect::<Vec<_>>();
        SysItemType::Clefs(segments)
    } else if value.starts_with("bl") {
        SysItemType::Other
    } else {
        let ids_and_durations = parse_parts(_cx, value)?;
        let max_duration = ids_and_durations.iter().map(|(_, d)| *d).max().unwrap();
        let ids = ids_and_durations.iter().map(|(id, _)| *id).collect::<Vec<_>>();
        SysItemType::Parts(ids, max_duration)
    };
    Ok(t)
}

pub fn parse_sysitems(cx: &CoreContext, value: &str) -> Result<Vec<ItemId>, Box<dyn std::error::Error>> {
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
            let stype = parse_sysitemtype(cx, s).expect("Could not parse sysitemtype");
            let s = SysItem { id, stype };
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
    use core::context::utils::check_sysitems_parts_integrity;

    use super::*;

    #[test]
    fn test_n() {
        let cx = CoreContext::new();
        let _ = parse_note(&cx, "b1,-3", 0, Duration::D8).unwrap();
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
    fn test_ss() {
        let cx = CoreContext::new();
        let ids = parse_sysitems(cx, "|clef G | 0 / 0,3 1 2 % 0 1b 2 3 |bl | 0 / 1").unwrap();
        let _check = check_sysitems_parts_integrity(cx, ids);
        dbg!(&cx);
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
