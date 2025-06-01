use core::{
    accidental::Accidental,
    context::CoreContext,
    duration::NoteDuration,
    head::{HeadItem, HeadType, RestType},
    note::{NoteItem, NoteType},
    part::complex::{Complex, ComplexType},
};
use graphics::rectangle::Rectangle;
use utils::f32_ext::{half::F32ExtHalf, round::F32ExtRound2};

use crate::{
    constants::{ACCIDENTAL_HEIGHT, ACCIDENTAL_WIDTH, HEAD_WIDTH_BLACK, HEAD_WIDTH_WHITE, HEAD_WIDTH_WHOLE, REST_WIDTH, SPACE, SPACE2, SPACE_HALF},
    scoreitem::{ScoreItem, ScoreRectangle, ScoreRectangles},
};

pub fn create_rectangles_complex(_cx: &CoreContext, _partidx: usize, _complex: &Complex) -> ScoreRectangles {
    let mut rectangles: ScoreRectangles = Vec::new();

    match _complex.ctype {
        ComplexType::Upper(ref note) => {
            let note_rectangles = create_rectangles_note(note);
            rectangles.extend(note_rectangles);

            let mut accidentals = collect_accidentals(_cx, note);
            sort_accidentals(&mut accidentals);
            let acc_rectangles = create_rectangles_accidentals(&accidentals);
            rectangles.extend(acc_rectangles);
        }
        ComplexType::Lower(ref note) => {
            let note_rectangles = create_rectangles_note(note);
            rectangles.extend(note_rectangles);

            let mut accidentals = collect_accidentals(_cx, note);
            sort_accidentals(&mut accidentals);
            let acc_rectangles = create_rectangles_accidentals(&accidentals);
            rectangles.extend(acc_rectangles);
        }
        ComplexType::UpperAndLower(ref upper, ref lower, _diff) => {
            let mut note_rectangles = create_rectangles_note(upper);
            note_rectangles.extend(create_rectangles_note(lower));
            rectangles.extend(note_rectangles);

            let mut accidentals = collect_accidentals(_cx, upper);
            accidentals.extend(collect_accidentals(_cx, lower));
            sort_accidentals(&mut accidentals);
            let acc_rectangles = create_rectangles_accidentals(&accidentals);
            rectangles.extend(acc_rectangles);
        }
    }

    rectangles
}

pub fn sort_accidentals(accidentals: &mut Vec<(i8, Accidental)>) -> &mut Vec<(i8, Accidental)> {
    accidentals.sort_by(|a, b| a.0.cmp(&b.0));
    accidentals
}

pub fn collect_accidentals(_cx: &CoreContext, _note: &NoteItem) -> Vec<(i8, Accidental)> {
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

fn create_rectangles_accidentals(accs: &[(i8, Accidental)]) -> ScoreRectangles {
    let mut rectangles: ScoreRectangles = Vec::new();
    for (accidx, (level, accidental)) in accs.iter().enumerate() {
        let x = (-ACCIDENTAL_WIDTH * (accidx as f32)) - ACCIDENTAL_WIDTH;
        let level_y: f32 = *level as f32 * SPACE_HALF;
        let rect: Rectangle = (x, (-ACCIDENTAL_HEIGHT.half() + level_y).r2(), ACCIDENTAL_WIDTH, ACCIDENTAL_HEIGHT);
        let item = match accidental {
            Accidental::Sharp => ScoreItem::AccidentalSharp,
            Accidental::Flat => ScoreItem::AccidentalFlat,
            Accidental::Natural => ScoreItem::AccidentalNatural,
            _ => continue, // Skip if no accidental
        };
        rectangles.push((rect, item));
    }
    rectangles
}

pub fn create_rectangles_note(_note: &NoteItem) -> ScoreRectangles {
    let mut rectangles: ScoreRectangles = Vec::new();
    match _note.ntype {
        NoteType::Heads(ref heads) => {
            for head in heads {
                rectangles.push(create_rectangle_head(&_note.duration, head));
            }
        }
        NoteType::Rest => {
            rectangles.push(create_rectangle_rest(&_note.duration));
        }
        NoteType::LyricItem => {
            println!("Note is LyricItem");
        }
    }
    rectangles
}

fn create_rectangle_head(duration: &NoteDuration, head: &HeadItem) -> ScoreRectangle {
    let level_y: f32 = head.level as f32 * SPACE_HALF;
    let rect: Rectangle = (0., -SPACE_HALF + level_y, get_head_width(duration), SPACE);
    let item: ScoreItem = ScoreItem::HeadBlack;
    (rect, item)
}

fn get_head_width(duration: &NoteDuration) -> f32 {
    match duration.get_head_type() {
        HeadType::White => HEAD_WIDTH_WHITE, // Example adjustment for white heads
        HeadType::Whole => HEAD_WIDTH_WHOLE, // No head
        _ => HEAD_WIDTH_BLACK,
    }
}

fn create_rectangle_rest(duration: &NoteDuration) -> ScoreRectangle {
    let rect: Rectangle = (0., -SPACE, REST_WIDTH, SPACE2);
    match duration.get_rest_type() {
        RestType::Quarter => (rect, ScoreItem::RestQuarter),
        RestType::Half => (rect, ScoreItem::RestHalf),
        RestType::Whole => (rect, ScoreItem::RestWhole),
        RestType::Eighth => (rect, ScoreItem::RestEighth),
        _ => todo!("unimpemented rest type"),
    }
}
