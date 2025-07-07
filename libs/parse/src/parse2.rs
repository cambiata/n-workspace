use core::{
    context::CoreContext,
    duration::{NoteDuration, SumDuration},
    hpart::VoiceType2,
    stems::stemitems::StemItemUtils,
    sysitem::SysItemTypeId,
};
use std::error::Error;

use crate::{parse::parse_notes, parse2utils::Parse2Utils};

pub struct Parse2;

impl Parse2 {
    pub fn sysitemlist2(cx: &CoreContext, value: &str, split_parts: bool) -> Result<(), Box<dyn Error>> {
        let mut value = value.trim();
        if value.starts_with("|") {
            value = value[1..].trim();
        }
        if value.ends_with("|") {
            value = &value[..value.len() - 1];
        }

        let items_str = value.split("|").collect::<Vec<_>>();
        // let mut max_parts_count = 0;
        let mut bpvmap: Vec<SysItemTypeId> = Vec::new();
        for (_item_idx, item_str) in items_str.iter().filter(|s| !s.trim().is_empty()).enumerate() {
            Parse2::sysitemtype2(cx, item_str, &mut bpvmap)?;
        }
        //------------------------------------------------
        let parts_config = Parse2Utils::get_parts_config(&bpvmap);
        let mut parts_count = parts_config.len();
        if split_parts {
            parts_count = Parse2Utils::split_double_voices(&mut bpvmap, parts_count);
        }

        Parse2Utils::correct_to_parts_count(&mut bpvmap, parts_count);
        Parse2Utils::create_columns_of_parts2(cx, &mut bpvmap, parts_count)?;
        Parse2Utils::create_rows_from_columns(cx)?;
        Parse2Utils::set_stemitems_directions(cx)?;
        Parse2Utils::calculate_head_positions(cx)?;
        Parse2Utils::map_notes_by_voices(cx)?;
        Parse2Utils::resolve_ties_from(cx)?;
        Parse2Utils::resolve_ties_to(cx)?;

        Ok(())
    }

    pub fn sysitemtype2(_cx: &CoreContext, value: &str, bpvmap: &mut Vec<SysItemTypeId>) -> Result<(), Box<dyn Error>> {
        let mut value = value.trim();
        if value.starts_with("|") {
            value = value[1..].trim();
        }
        //-----------------------------------
        if value.starts_with("clef") {
            // Clefs
            let clef_segments = value.split(" ").skip(1).filter(|s| !s.is_empty()).map(|s| s.to_string()).collect::<Vec<_>>();
            bpvmap.push(SysItemTypeId::Clefs(clef_segments));
        } else if value.starts_with("bl") {
            // Barlines
            bpvmap.push(SysItemTypeId::Barlines(vec!["Single".to_string()]));
        } else {
            // Parts, voices
            if value.starts_with("/") {
                value = value[1..].trim();
            }
            let mut pa: Vec<Vec<String>> = vec![];
            let part_segments = value.split("/").filter(|s| !s.is_empty()).collect::<Vec<_>>();
            for (_part_idx, part_segment) in part_segments.iter().enumerate() {
                let voice_segments = part_segment.split("%").filter(|s| !s.is_empty()).collect::<Vec<_>>();
                let mut va: Vec<String> = vec![];
                for (_voice_idx, voice_segment) in voice_segments.iter().enumerate() {
                    let mut v = voice_segment.trim().to_string();
                    if v.is_empty() {
                        v = "Voice-added1".to_string(); // Default voice
                    }
                    va.push(v);
                }
                pa.push(va);
            }
            bpvmap.push(SysItemTypeId::Parts(pa.clone()));
        };

        Ok(())
    }

    pub fn voicetype(cx: &CoreContext, value: &str, default_duration: SumDuration) -> Result<VoiceType2, Box<dyn Error>> {
        let value = value.trim();
        let vtype = if value.starts_with("bp") {
            VoiceType2::Barpause(default_duration)
        } else if value.starts_with("Voice") {
            VoiceType2::Barpause(default_duration)
        } else {
            let (note_ids, duration) = parse_notes(cx, value)?;

            let pattern_values = vec![NoteDuration::D4];
            let stemitem_ids = StemItemUtils::create_stem_items_from_notes(cx, &note_ids, duration, pattern_values).unwrap();
            VoiceType2::NoteIds { note_ids, duration, stemitem_ids }
        };

        Ok(vtype)
    }
}
