use core::{
    barline::BarlineType,
    clef::ClefSignature,
    context::CoreContext,
    hpart::{HPartItemsColumnType, HPartType},
};
use std::panic;

use grid::griditem::GridItemType;

use crate::{
    constants::{BARLINE_DOUBLE_WIDTH, BARLINE_FINAL_WIDTH, BARLINE_WIDTH, CLEF_WIDTH, SPACE2, SPACE4},
    glyphitem::{GlyphItem, GlyphRectangle},
    scorecontext::ScoreContext,
};

pub struct ScoreUtils2;
impl ScoreUtils2 {
    pub fn build(scx: &ScoreContext, cx: &CoreContext) -> Result<(), Box<dyn std::error::Error>> {
        let cx_complexes = cx.complexes.borrow();

        for (column_idx, item) in cx.columns.borrow().iter().enumerate() {
            match item.hptype {
                HPartItemsColumnType::Clefs(ref ids) => {
                    Self::build_clefs(scx, cx, ids.clone())?;
                }
                HPartItemsColumnType::Barlines(ref ids) => {
                    Self::build_barlines(scx, cx, ids.clone())?;
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
                        column_griditems.push(GridItemType::Rectangles(vec![(rect, glyph.clone())]));
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
                let item: GlyphRectangle = (rect, glyph.clone());
                column_griditems.push(GridItemType::Rectangles(vec![item.clone()]));
            } else {
                panic!("Expected HPartType::Barline, found {:?}", hpart.hptype);
            }
        });
        scx.grid_columns.borrow_mut().push(column_griditems);
        Ok(())
    }
}
