use core::{
    accidental::Accidental,
    duration::NoteDuration,
    head::{HeadItem, HeadType, HeadVariant},
    note::{NoteItem, NoteType},
    stems::stemitems::StemHeadPosition,
};
use graphics::rectangle::{rectangle_overlap_x, Rectangle};
use std::collections::BTreeMap;
use utils::f32_ext::{half::F32ExtHalf, round::F32ExtRound2};

use crate::{
    constants::*,
    glyphitem::{GlyphItem, GlyphRectangle},
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

#[allow(unused_assignments)]
pub fn create_glyphsrectangles_accidentals(accs: &[(i8, Accidental)], rectangles: &mut Vec<(Rectangle, GlyphItem)>) -> f32 {
    // let mut rectangles: ComplexGlyphsRectangles = Vec::new();

    let mut altidx = 0; // Alternate index for even/odd handling

    let mut leftmost_x: f32 = 0.0; // Leftmost x position for the accidentals

    for accidx in 0..accs.len() {
        if (&accidx % 2) == 0 {
            altidx = accidx.div_ceil(2);
            // println!("Even index: {} {}", accidx, altidx);
        } else {
            altidx = &accs.len() - accidx.div_ceil(2);
            // println!("Odd index: {} {}", accidx, altidx);
        }

        let (level, accidental) = &accs[altidx];

        let item = match accidental {
            Accidental::Sharp => GlyphItem::Accidental(accidental.clone()),
            Accidental::Flat => GlyphItem::Accidental(accidental.clone()),
            Accidental::Natural => GlyphItem::Accidental(accidental.clone()),
            _ => continue, // Skip if no accidental
        };
        let width = match accidental {
            Accidental::Sharp => ACCIDENTAL_WIDTH_WIDE, // Natural is wider
            _ => ACCIDENTAL_WIDTH_NARROW,
        };

        let level_y: f32 = *level as f32 * SPACE_HALF;
        let mut rect: Rectangle = (0.0, (-ACCIDENTAL_HEIGHT.half() + level_y).r2(), width, ACCIDENTAL_HEIGHT);

        // let overlap = rectangles.iter().any(|(r, _)| r.overlaps(&rect));
        let overlap = rectangles_overlap_left(rectangles, &rect);
        rect.0 = -overlap;

        rectangles.push((rect, item));
        leftmost_x = leftmost_x.min(rect.0);
    }

    leftmost_x
}

pub fn rectangles_overlap_left(lefts: &[(Rectangle, GlyphItem)], right: &Rectangle) -> f32 {
    let mut result: f32 = 0.;
    lefts.iter().for_each(|left| {
        let ol = rectangle_overlap_x(*right, left.0);
        result = result.max(ol);
    });
    result
}

pub fn create_glyphsrectangles_note(_note: &NoteItem, map_head_position: &BTreeMap<usize, StemHeadPosition>, rectangles: &mut Vec<(Rectangle, GlyphItem)>) -> f32 {
    // let mut rectangles: ComplexGlyphsRectangles = Vec::new();
    let mut leftmost_x: f32 = 0.0; // Leftmost x position for the note glyphs
    match _note.ntype {
        NoteType::Heads(ref heads) => {
            for head in heads {
                let rect = create_glyphrectangle_head(&_note.duration, head, map_head_position);
                leftmost_x = leftmost_x.min(rect.0 .0);
                rectangles.push(rect);
            }
        }
        NoteType::Rest => {
            rectangles.push(create_glyphrectangle_rest(&_note.duration));
        }
        NoteType::LyricItem => {
            println!("Note is LyricItem");
        }
    }
    leftmost_x
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
