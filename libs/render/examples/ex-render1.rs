// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-render1"

use core::context::CoreContext;
use std::fs;

use grid::{gridcontext::GridContext, griditem::GridItemType};
use parse::parse2::Parse2;
use render::gridrender::Render;
use score::{build::ScoreUtils2, glyphitem::GlyphItem, scorecontext::ScoreContext};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cx = CoreContext::new();
    let _ = Parse2::sysitemlist2(cx, "clef G F | bl", false).unwrap();

    let scx = ScoreContext::new();
    ScoreUtils2::build(&scx, &cx)?;
    dbg!(&scx.grid_columns.borrow());

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
    fs::write("libs/render/examples/ex-render1.svg", Render::render_gridcontext_with_glyphitem(gcx)).unwrap();

    Ok(())
}
