// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-render1"

use core::context::CoreContext;
use graphics::graphicitem::GraphicItems;
use grid::{gridcontext::GridContext, griditem::GridItemType};
use parse::parse2::Parse2;
use render::gridrender::Render;
use score::{build::BuildScore, glyphitem::GlyphItem, scorecontext::ScoreContext};
use std::fs;
use svg::svg_renderer::SvgBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cx = CoreContext::new();
    // let _ = Parse2::sysitemlist2(cx, "clef G F | D8 -3 n-1 #4 3 r -2 -2 -3 / 0 ", false).unwrap();
    let _ = Parse2::sysitemlist2(cx, "clef G G | 2 3 4 5 6 7 ", false).unwrap();

    // let _ = Parse2::sysitemlist2(cx, "|clef G |0 -1 -2 -3 -4 -5 -6 -7  5 4 3 2 1 0 -1 -2 -3 -4 -5 % 1 1 2 3 4 5 6 7 ", false).unwrap();

    // dbg!(&cx.stemitems.borrow());

    let scx = ScoreContext::new();
    BuildScore::build(&scx, &cx)?;

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

    // calculate distances
    let allotments: Vec<f32> = scx.grid_column_allotment.borrow().to_vec();
    gcx.handle_column_spacing(&allotments, 2.3)?;
    gcx.handle_row_heights()?;

    // create graphic items
    let mut graphic_items = GraphicItems::new();
    let notelines = Render::render_notelines(&gcx);
    graphic_items.extend(notelines);
    let glyphitems = Render::render_music_glyphitems(&gcx);
    graphic_items.extend(glyphitems);

    // save to svg
    let svg_string = SvgBuilder::new().build(graphic_items, None);
    fs::write("libs/render/examples/ex-render1.svg", svg_string)?;

    Ok(())
}
