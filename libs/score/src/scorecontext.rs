use core::stems::stemitems::StemHeadPosition;
use std::{cell::RefCell, collections::BTreeMap};

use grid::griditem::GridItemType;

use crate::glyphitem::GlyphItem;

#[derive(Debug)]
pub struct ScoreContext {
    pub grid_columns: RefCell<Vec<Vec<GridItemType<GlyphItem>>>>,
    pub grid_column_sysitem_ids: RefCell<Vec<usize>>,
    pub grid_column_allotment: RefCell<Vec<f32>>,
    pub map_head_position: RefCell<BTreeMap<usize, StemHeadPosition>>,
}

impl ScoreContext {
    pub fn new() -> &'static ScoreContext {
        let scx = ScoreContext {
            grid_columns: RefCell::new(Vec::new()),
            grid_column_sysitem_ids: RefCell::new(Vec::new()),
            grid_column_allotment: RefCell::new(Vec::new()),
            map_head_position: RefCell::new(BTreeMap::new()),
        };
        Box::leak(Box::new(scx))
    }

    // Vec<Vec<GridItemType<GlyphItem>>>
}
