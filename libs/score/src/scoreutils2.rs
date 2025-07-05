use core::{
    complex,
    context::CoreContext,
    hpart::{HPartMusicType, HPartType},
};
use std::collections::BTreeMap;

use crate::scorecontext::ScoreContext;

pub struct ScoreUtils2;
impl ScoreUtils2 {
    pub fn build(scx: &ScoreContext, cx: &CoreContext) {
        let cx_hparts = cx.hparts.borrow();
        let cx_complexes = cx.complexes.borrow();

        let map_complex: BTreeMap<usize, Vec<Option<usize>>> = BTreeMap::new();

        for (column_idx, item) in cx.columns.borrow().iter().enumerate() {
            dbg!(item);
        }
    }
}
