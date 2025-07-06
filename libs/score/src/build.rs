use core::{
    accidental::Accidental,
    barline::BarlineType,
    clef::ClefSignature,
    complex::{self, ComplexType},
    context::CoreContext,
    duration::NoteDuration,
    head::{HeadItem, HeadType, HeadVariant},
    hpart::{HPartItemsColumnType, HPartType},
    note::{NoteItem, NoteType},
    stems::stemitems::StemHeadPosition,
};

use graphics::{
    color::Color,
    rectangle::{rectangle_overlap_x, Rectangle},
};
use grid::griditem::GridItemType;

use utils::f32_ext::{half::F32ExtHalf, round::F32ExtRound2};

use crate::{
    buildutils::BuildUtils,
    constants::{
        ACCIDENTAL_HEIGHT, ACCIDENTAL_WIDTH_NARROW, ACCIDENTAL_WIDTH_WIDE, BARLINE_DOUBLE_WIDTH, BARLINE_FINAL_WIDTH, BARLINE_WIDTH, CLEF_WIDTH, HEAD_WIDTH_BLACK, HEAD_WIDTH_WHITE, HEAD_WIDTH_WHOLE,
        SPACE, SPACE2, SPACE4, SPACE_BEFORE_FIRST_NOTE_IN_BAR, SPACE_HALF,
    },
    glyphitem::{GlyphItem, GlyphRectangle},
    scorecontext::ScoreContext,
};

pub struct Build;
impl Build {
    pub fn build(scx: &ScoreContext, cx: &CoreContext) -> Result<(), Box<dyn std::error::Error>> {
        for (column_idx, item) in cx.columns.borrow().iter().enumerate() {
            match item.hptype {
                HPartItemsColumnType::Clefs(ref ids) => {
                    Self::build_clefs(scx, cx, ids.clone())?;
                }
                HPartItemsColumnType::Barlines(ref ids) => {
                    Self::build_barlines(scx, cx, ids.clone())?;
                }
                HPartItemsColumnType::Musics(ref ids) => {
                    Self::build_musics(scx, cx, ids.clone())?;
                }

                _ => {}
            }
        }

        Ok(())
    }

    fn build_clefs(scx: &ScoreContext, cx: &CoreContext, ids: Vec<usize>) -> Result<(), Box<dyn std::error::Error>> {
        let cx_hparts = cx.hparts.borrow();
        let hparts = ids.iter().map(|id| &cx_hparts[*id]).collect::<Vec<_>>();

        let mut column_griditems: Vec<GridItemType<GlyphItem>> = Vec::new();
        hparts.iter().for_each(|hpart| {
            if let HPartType::Clef(clef) = &hpart.hptype {
                match clef {
                    ClefSignature::None => {
                        column_griditems.push(GridItemType::Empty);
                    }
                    _ => {
                        let glyph: GlyphItem = GlyphItem::Clef(clef.clone());
                        let rect = (0.0, -SPACE2, CLEF_WIDTH, SPACE4);
                        column_griditems.push(GridItemType::Rectangles(vec![(rect, glyph)]));
                    }
                }
            } else {
                panic!("Expected HPartType::Clef, found {:?}", hpart.hptype);
            }
        });
        scx.grid_columns.borrow_mut().push(column_griditems);
        Ok(())
    }

    fn build_barlines(scx: &ScoreContext, cx: &CoreContext, ids: Vec<usize>) -> Result<(), Box<dyn std::error::Error>> {
        let cx_hparts = cx.hparts.borrow();
        let hparts = ids.iter().map(|id| &cx_hparts[*id]).collect::<Vec<_>>();

        let mut column_griditems: Vec<GridItemType<GlyphItem>> = Vec::new();
        hparts.iter().for_each(|hpart| {
            if let HPartType::Barline(btype) = &hpart.hptype {
                let rect = match btype {
                    BarlineType::Double => (0.0, -SPACE2, BARLINE_DOUBLE_WIDTH, SPACE4),
                    BarlineType::Final => (0.0, -SPACE2, BARLINE_FINAL_WIDTH, SPACE4),
                    _ => (0.0, -SPACE2, BARLINE_WIDTH, SPACE4),
                };
                let glyph: GlyphItem = GlyphItem::Barline(btype.clone());
                let item: GlyphRectangle = (rect, glyph);
                column_griditems.push(GridItemType::Rectangles(vec![item.clone()]));
            } else {
                panic!("Expected HPartType::Barline, found {:?}", hpart.hptype);
            }
        });
        scx.grid_columns.borrow_mut().push(column_griditems);
        Ok(())
    }

    fn build_musics(scx: &ScoreContext, cx: &CoreContext, ids: Vec<usize>) -> Result<(), Box<dyn std::error::Error>> {
        let cx_hparts = cx.hparts.borrow();
        let cx_complexes = cx.complexes.borrow();
        let hparts = ids.iter().map(|id| &cx_hparts[*id]).collect::<Vec<_>>();
        let parts_count = hparts.len();
        let (positions, map, map_ids) = BuildUtils::get_complexes_information(cx, &hparts)?;

        for position in positions {
            // each position corresponds to a column in the grid
            let mut column_griditems: Vec<GridItemType<GlyphItem>> = Vec::new();

            for part_idx in 0..parts_count {
                if map_ids.contains_key(&(part_idx, position)) {
                    if let Some(complex_id) = map_ids.get(&(part_idx, position)) {
                        dbg!(position, part_idx, complex_id);
                        let complex = &cx_complexes[*complex_id];
                        dbg!(complex);
                        let rects = Build::build_complex(cx, complex, part_idx, position)?;

                        column_griditems.push(GridItemType::Rectangles(rects));
                    } else {
                        panic!("Complex ID not found for part_idx: {}, position: {}", part_idx, position);
                    }
                } else {
                    column_griditems.push(GridItemType::Empty);
                }
            }
            scx.grid_columns.borrow_mut().push(column_griditems);
        }
        Ok(())
    }

    fn build_complex(cx: &CoreContext, complex: &complex::Complex, part_idx: usize, position: usize) -> Result<Vec<(Rectangle, GlyphItem)>, Box<dyn std::error::Error>> {
        let mut rects: Vec<(Rectangle, GlyphItem)> = Vec::new();

        match &complex.ctype {
            ComplexType::Upper(note) | ComplexType::Lower(note) => {
                let rs = Build::build_notetype(cx, note, part_idx, position)?;
                let leftmost_head_x: f32 = leftmost_x(&rs);
                rects.extend(rs);

                let mut accidentals = collect_accidentals(note);
                sort_accidentals(&mut accidentals);
                let leftmost_accidental_x = create_glyphsrectangles_accidentals(&accidentals, &mut rects);

                if note.position == 0 {
                    rects.push(create_space_rectangle_for_first_note_in_bar(leftmost_accidental_x.min(leftmost_head_x)));
                }
            }
            ComplexType::UpperAndLower(upper, lower, diffx) => {
                let rs = Build::build_notetype(cx, upper, part_idx, position)?;
                let leftmost_upper_x: f32 = leftmost_x(&rs);
                rects.extend(rs);
                let rs = Build::build_notetype(cx, lower, part_idx, position)?;
                let leftmost_lower_x: f32 = leftmost_x(&rs);
                rects.extend(rs);

                let mut accidentals = collect_accidentals(upper);
                accidentals.extend(collect_accidentals(lower));
                sort_accidentals(&mut accidentals);

                let leftmost_accidental_x = create_glyphsrectangles_accidentals(&accidentals, &mut rects);

                if upper.position == 0 {
                    rects.push(create_space_rectangle_for_first_note_in_bar(leftmost_accidental_x.min(leftmost_upper_x.min(leftmost_lower_x))));
                }
            }
        }

        // let rect: Rectangle = (0., -SPACE, SPACE, SPACE);
        // let glyph: GlyphItem = GlyphItem::XRect(Color::DodgerBlue);
        // rects.push((rect, glyph));

        // let rect: Rectangle = (SPACE_HALF, -SPACE_HALF, SPACE, SPACE);
        // let glyph: GlyphItem = GlyphItem::XRect(Color::Tomato);
        // rects.push((rect, glyph));

        Ok(rects)
    }

    fn build_notetype(cx: &CoreContext, note: &NoteItem, part_idx: usize, position: usize) -> Result<Vec<(Rectangle, GlyphItem)>, Box<dyn std::error::Error>> {
        let mut rects: Vec<(Rectangle, GlyphItem)> = Vec::new();

        match note.ntype {
            core::note::NoteType::Heads(ref heads) => {
                let rs = Build::build_heads(cx, note, heads, part_idx, position)?;
                rects.extend(rs);
            }
            core::note::NoteType::Rest => {
                let rs = Build::build_rest(cx, note, part_idx, position)?;
                rects.extend(rs);
            }
            core::note::NoteType::LyricItem => {
                println!("Note is LyricItem");
            }
        }

        Ok(rects)
    }

    fn build_heads(cx: &CoreContext, note: &NoteItem, heads: &[HeadItem], part_idx: usize, position: usize) -> Result<Vec<(Rectangle, GlyphItem)>, Box<dyn std::error::Error>> {
        let mut rects: Vec<(Rectangle, GlyphItem)> = Vec::new();
        for head in heads {
            let rs = Build::build_head(cx, note, head, part_idx, position)?;
            rects.extend(rs);
        }
        Ok(rects)
    }

    fn build_rest(cx: &CoreContext, note: &NoteItem, part_idx: usize, position: usize) -> Result<Vec<(Rectangle, GlyphItem)>, Box<dyn std::error::Error>> {
        let mut rects: Vec<(Rectangle, GlyphItem)> = Vec::new();
        Ok(rects)
    }

    fn build_head(cx: &CoreContext, note: &NoteItem, head: &HeadItem, part_idx: usize, position: usize) -> Result<Vec<(Rectangle, GlyphItem)>, Box<dyn std::error::Error>> {
        let mut rects: Vec<(Rectangle, GlyphItem)> = Vec::new();

        let cx_map_head_position = cx.map_head_position.borrow();
        let head_x: f32 = if !cx_map_head_position.contains_key(&head.id) {
            0.
        } else {
            match cx_map_head_position.get(&head.id).cloned().unwrap_or(StemHeadPosition::Center) {
                StemHeadPosition::Center => 0.,
                StemHeadPosition::Left => -get_head_width(&note.duration),
                StemHeadPosition::Right => get_head_width(&note.duration),
            }
        };
        let level_y: f32 = head.level as f32 * SPACE_HALF;
        let rect: Rectangle = (head_x, -SPACE_HALF + level_y, get_head_width(&note.duration), SPACE);
        let item: GlyphItem = GlyphItem::Notehead(note.duration.get_head_type(), HeadVariant::Normal);
        rects.push((rect, item));

        Ok(rects)
    }
}

fn leftmost_x(rs: &[(Rectangle, GlyphItem)]) -> f32 {
    rs.iter().map(|((x, _, _, _), _)| *x).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0)
}

fn get_head_width(duration: &NoteDuration) -> f32 {
    match duration.get_head_type() {
        HeadType::White => HEAD_WIDTH_WHITE, // Example adjustment for white heads
        HeadType::Whole => HEAD_WIDTH_WHOLE, // No head
        _ => HEAD_WIDTH_BLACK,
    }
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

#[allow(unused_assignments)]
pub fn create_glyphsrectangles_accidentals(accs: &[(i8, Accidental)], rectangles: &mut Vec<(Rectangle, GlyphItem)>) -> f32 {
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

fn create_space_rectangle_for_first_note_in_bar(left_x: f32) -> ((f32, f32, f32, f32), GlyphItem) {
    ((left_x - SPACE_BEFORE_FIRST_NOTE_IN_BAR, -SPACE, SPACE_BEFORE_FIRST_NOTE_IN_BAR, SPACE2), GlyphItem::XRect(Color::Gray))
}
