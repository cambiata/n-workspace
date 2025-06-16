use core::{
    accidental::Accidental,
    duration::NoteDuration,
    head::{HeadItem, HeadType, HeadVariant},
    note::{NoteItem, NoteType},
    stems::stemitems::StemHeadPosition,
};
use graphics::rectangle::Rectangle;
use std::collections::BTreeMap;
use utils::f32_ext::{half::F32ExtHalf, round::F32ExtRound2};

use crate::{
    constants::{ACCIDENTAL_HEIGHT, ACCIDENTAL_WIDTH, HEAD_WIDTH_BLACK, HEAD_WIDTH_WHITE, HEAD_WIDTH_WHOLE, REST_WIDTH, SPACE, SPACE2, SPACE_HALF},
    glyphitem::{ComplexGlyphsRectangles, GlyphItem, GlyphRectangle},
};

pub fn sort_accidentals(accidentals: &mut Vec<(i8, Accidental)>) -> &mut Vec<(i8, Accidental)> {
    accidentals.sort_by(|a, b| a.0.cmp(&b.0));
    accidentals
}

pub fn collect_accidentals(_note: &NoteItem) -> Vec<(i8, Accidental)> {
    let mut accidentals: Vec<(i8, Accidental)> = Vec::new();
    match _note.ntype {
        NoteType::Heads(ref heads) => {
            for head in heads {
                match head.accidental {
                    Accidental::None => {}
                    _ => {
                        accidentals.push((head.level, head.accidental.clone()));
                    }
                }
            }
        }
        _ => {}
    }
    accidentals
}

pub fn create_glyphsrectangles_accidentals(accs: &[(i8, Accidental)]) -> ComplexGlyphsRectangles {
    let mut rectangles: ComplexGlyphsRectangles = Vec::new();
    for (accidx, (level, accidental)) in accs.iter().enumerate() {
        let x = (-ACCIDENTAL_WIDTH * (accidx as f32)) - ACCIDENTAL_WIDTH;
        let level_y: f32 = *level as f32 * SPACE_HALF;
        let rect: Rectangle = (x, (-ACCIDENTAL_HEIGHT.half() + level_y).r2(), ACCIDENTAL_WIDTH, ACCIDENTAL_HEIGHT);
        let item = match accidental {
            Accidental::Sharp => GlyphItem::Accidental(accidental.clone()),
            Accidental::Flat => GlyphItem::Accidental(accidental.clone()),
            Accidental::Natural => GlyphItem::Accidental(accidental.clone()),
            _ => continue, // Skip if no accidental
        };
        rectangles.push((rect, item));
    }
    rectangles
}

pub fn create_glyphsrectangles_note(_note: &NoteItem, map_head_position: &BTreeMap<usize, StemHeadPosition>) -> ComplexGlyphsRectangles {
    let mut rectangles: ComplexGlyphsRectangles = Vec::new();
    match _note.ntype {
        NoteType::Heads(ref heads) => {
            for head in heads {
                rectangles.push(create_glyphrectangle_head(&_note.duration, head, map_head_position));
            }
        }
        NoteType::Rest => {
            rectangles.push(create_glyphrectangle_rest(&_note.duration));
        }
        NoteType::LyricItem => {
            println!("Note is LyricItem");
        }
    }
    rectangles
}

fn create_glyphrectangle_head(duration: &NoteDuration, head: &HeadItem, map_head_position: &BTreeMap<usize, StemHeadPosition>) -> GlyphRectangle {
    let head_x: f32 = if !map_head_position.contains_key(&head.id) {
        0.
    } else {
        match map_head_position.get(&head.id).cloned().unwrap_or(StemHeadPosition::Center) {
            StemHeadPosition::Center => 0.,
            StemHeadPosition::Left => -get_head_width(duration),
            StemHeadPosition::Right => get_head_width(duration),
        }
    };

    let level_y: f32 = head.level as f32 * SPACE_HALF;
    let rect: Rectangle = (head_x, -SPACE_HALF + level_y, get_head_width(duration), SPACE);
    // dbg!(&rect);
    let item: GlyphItem = GlyphItem::Notehead(duration.get_head_type(), HeadVariant::Normal);
    (rect, item)
}

fn get_head_width(duration: &NoteDuration) -> f32 {
    match duration.get_head_type() {
        HeadType::White => HEAD_WIDTH_WHITE, // Example adjustment for white heads
        HeadType::Whole => HEAD_WIDTH_WHOLE, // No head
        _ => HEAD_WIDTH_BLACK,
    }
}

fn create_glyphrectangle_rest(duration: &NoteDuration) -> GlyphRectangle {
    let rect: Rectangle = (0., -SPACE, REST_WIDTH, SPACE2);
    (rect, GlyphItem::Rest(duration.get_rest_type()))
}
