use core::{
    accidental::Accidental,
    complex::{Complex, ComplexType},
    duration::NoteDuration,
    head::{HeadItem, HeadType, RestType},
    note::{NoteItem, NoteType},
};
use graphics::rectangle::Rectangle;
use utils::f32_ext::{half::F32ExtHalf, round::F32ExtRound2};

use crate::{
    constants::{ACCIDENTAL_HEIGHT, ACCIDENTAL_WIDTH, HEAD_WIDTH_BLACK, HEAD_WIDTH_WHITE, HEAD_WIDTH_WHOLE, REST_WIDTH, SPACE, SPACE2, SPACE_HALF},
    glyphitem::{ComplexGlyphsRectangles, GlyphItem, GlyphRectangle},
};

pub fn create_glyphsrectangles_complex(_partidx: usize, _complex: &Complex) -> ComplexGlyphsRectangles {
    let mut rectangles: ComplexGlyphsRectangles = Vec::new();

    match _complex.ctype {
        ComplexType::Upper(ref note) => {
            // note
            let note_rectangles = create_glyphsrectangles_note(note);
            rectangles.extend(note_rectangles);

            // accidentals
            let mut accidentals = collect_accidentals(note);
            sort_accidentals(&mut accidentals);
            let acc_rectangles = create_glyphsrectangles_accidentals(&accidentals);
            rectangles.extend(acc_rectangles);
        }
        ComplexType::Lower(ref note) => {
            // note
            let note_rectangles = create_glyphsrectangles_note(note);
            rectangles.extend(note_rectangles);

            // accidentals
            let mut accidentals = collect_accidentals(note);
            sort_accidentals(&mut accidentals);
            let acc_rectangles = create_glyphsrectangles_accidentals(&accidentals);
            rectangles.extend(acc_rectangles);
        }
        ComplexType::UpperAndLower(ref upper, ref lower, _diff) => {
            // note
            let mut note_rectangles = create_glyphsrectangles_note(upper);
            note_rectangles.extend(create_glyphsrectangles_note(lower));
            rectangles.extend(note_rectangles);

            // accidentals
            let mut accidentals = collect_accidentals(upper);
            accidentals.extend(collect_accidentals(lower));
            sort_accidentals(&mut accidentals);
            let acc_rectangles = create_glyphsrectangles_accidentals(&accidentals);
            rectangles.extend(acc_rectangles);
        }
    }

    rectangles
}

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

fn create_glyphsrectangles_accidentals(accs: &[(i8, Accidental)]) -> ComplexGlyphsRectangles {
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

pub fn create_glyphsrectangles_note(_note: &NoteItem) -> ComplexGlyphsRectangles {
    let mut rectangles: ComplexGlyphsRectangles = Vec::new();
    match _note.ntype {
        NoteType::Heads(ref heads) => {
            for head in heads {
                rectangles.push(create_glyphrectangle_head(&_note.duration, head));
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

fn create_glyphrectangle_head(duration: &NoteDuration, head: &HeadItem) -> GlyphRectangle {
    let level_y: f32 = head.level as f32 * SPACE_HALF;
    let rect: Rectangle = (0., -SPACE_HALF + level_y, get_head_width(duration), SPACE);
    let item: GlyphItem = GlyphItem::HeadBlack;
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
    match duration.get_rest_type() {
        RestType::Quarter => (rect, GlyphItem::RestQuarter),
        RestType::Half => (rect, GlyphItem::RestHalf),
        RestType::Whole => (rect, GlyphItem::RestWhole),
        RestType::Eighth => (rect, GlyphItem::RestEighth),
        _ => todo!("unimpemented rest type"),
    }
}
