// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-render1"

use core::context::CoreContext;
use grid::{gridcontext::GridContext, griditem::GridItemType};
use parse::parse2::Parse2;
use render::gridrender::Render;
use score::{build::BuildScore, glyphitem::GlyphItem, scorecontext::ScoreContext};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cx = CoreContext::new();
    let _ = Parse2::sysitemlist2(cx, "clef G | -3  3  ", false).unwrap();
    // let _ = Parse2::sysitemlist2(cx, "|clef G |0 -1 -2 -3 -4 -5 -6 -7  5 4 3 2 1 0 -1 -2 -3 -4 -5 % 1 1 2 3 4 5 6 7 ", false).unwrap();

    let scx = ScoreContext::new();
    BuildScore::build(&scx, &cx)?;

    dbg!(&cx.map_noteid_stemitemlevels);

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
    gcx.add_items(items2)?;
    let allotments: Vec<f32> = scx.grid_column_allotment.borrow().to_vec();
    gcx.handle_column_spacing(&allotments)?;

    fs::write("libs/render/examples/ex-render1.svg", Render::render_gridcontext_with_glyphitem(gcx)).unwrap();

    Ok(())
}
