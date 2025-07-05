use core::{
    barline::BarlineType,
    clef::ClefSignature,
    complex::{Complex, ComplexInfo, ComplexType},
    duration::SumDuration,
    part::PartId,
    stems::{
        headpositions::HeadPositionUtils,
        stemitems::{StemHeadPosition, StemItem},
    },
    sysitem::{SysItem, SysItemType},
};
use std::{cell::RefCell, collections::BTreeMap};

use graphics::{color::Color, rectangle::Rectangle};
use grid::griditem::GridItemType;

use crate::{
    complex::{collect_accidentals, create_glyphsrectangles_accidentals, create_glyphsrectangles_note, sort_accidentals},
    constants::{BARLINE_DOUBLE_WIDTH, BARLINE_FINAL_WIDTH, BARLINE_WIDTH, CLEF_WIDTH, SPACE, SPACE2, SPACE4, SPACE_BEFORE_FIRST_NOTE_IN_BAR},
    glyphitem::{ComplexGlyphsRectangles, GlyphItem, GlyphRectangle, PartGlyphsRectangles, SysitemGlyphsRectangles},
};

#[derive(Debug)]
pub struct ScoreContext {
    pub grid_columns: RefCell<Vec<Vec<GridItemType<GlyphItem>>>>,
    pub grid_column_sysitem_ids: RefCell<Vec<usize>>,
    pub map_head_position: RefCell<BTreeMap<usize, StemHeadPosition>>,
}

impl ScoreContext {
    pub fn new() -> &'static ScoreContext {
        let scx = ScoreContext {
            grid_columns: RefCell::new(Vec::new()),
            grid_column_sysitem_ids: RefCell::new(Vec::new()),
            map_head_position: RefCell::new(BTreeMap::new()),
        };
        Box::leak(Box::new(scx))
    }

    // Vec<Vec<GridItemType<GlyphItem>>>
}
