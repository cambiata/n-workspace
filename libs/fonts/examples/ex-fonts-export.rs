use std::fs;

use fonts::fontcontext::FontContext;
use svg::svg_renderer::SvgBuilder;

pub fn main() {
    let fcx = FontContext::new();
    let items = fcx.get_music_string_items(50.0, "N");
    // let items = fcx.get_sansserif_string_items(50.0, "Hejsan hoppsan Lönedåre");

    let svg = SvgBuilder::new().build(items, None);
    fs::write("libs/fonts/examples/ex-fonts-export.svg", svg).unwrap();
}
