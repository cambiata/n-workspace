// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-render2"
use core::context::CoreContext;
use std::fs;

use grid::{gridcontext::GridContext, griditem::GridItemType};
use parse::parse::parse_sysitemlist;
use render::gridrender::render_gridcontext_with_glyphitem;
use score::{glyphitem::GlyphItem, scorecontext::ScoreContext, scoreutils::ScoreUtils};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cx = CoreContext::new();

    // let _ = parse_sysitems(cx, "|clef G F| -3,3 #3,b-1 b0 / d2 0 #-1,b2").unwrap();
    // let _ = parse_sysitemlist(cx, "|clef G F |#-1,4n,-5#,6b b-1,#0 1 / 0 d2 0").unwrap();

    let _ = parse_sysitemlist(cx, "|clef G G |-3,-2 % #5 / 0 ").unwrap();
    let mut scx = ScoreContext::new();
    ScoreUtils::build_stemitems_headpositions(scx, &cx.stemitems.borrow())?;
    ScoreUtils::build_sysitems(scx, &cx.sysitems.borrow(), &cx.complexes.borrow())?;

    //-------------------------------------------------
    // Turn 180 degrees...
    let items = scx.grid_columns.borrow().to_vec();
    let mut items2: Vec<Vec<GridItemType<GlyphItem>>> = Vec::new();
    let rows = items[0].len();
    for row in 0..rows {
        let mut rowitems = Vec::new();
        for col in 0..items.len() {
            rowitems.push(items[col][row].clone());
        }
        items2.push(rowitems);
    }
    //-------------------------------------------------

    let gcx = GridContext::<GlyphItem>::new();
    gcx.add_items(items2);
    gcx.calculate_minimal_col_spacing();
    fs::write("libs/render/examples/ex-render2.svg", render_gridcontext_with_glyphitem(gcx)).unwrap();
    Ok(())
}
