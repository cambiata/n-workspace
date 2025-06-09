use std::{
    collections::{BTreeMap, HashMap},
    fs,
    str::Chars,
};

use fonts::fontcontext::FontContext;
use rusttype::{GlyphId, IntoGlyphId, Scale};
use svg::svg_renderer::SvgBuilder;

pub fn main() {
    let mut data: HashMap<u32, String> = HashMap::from([
        (35, "ACCIDENTAL_SHARP".to_string()),
        (38, "CLEF_TREBLE".to_string()),
        (63, "CLEF_BASS".to_string()),
        (66, "CLEF_ALTO".to_string()),
        (110, "NOTHEAD_WHOLE".to_string()),
        (729, "NOTEHEAD_WHITE".to_string()),
        (339, "NOTEHEAD_BLACK".to_string()),
        (339, "PAUSE_QUARTER".to_string()),
        (61, "FIVELINES".to_string()),
    ]);

    const SCALE: f32 = 99.0;

    let fcx = FontContext::new();
    let mut lines = vec!["use graphics::path::PathSegment::{self, L, M, Q, Z};".to_string()];

    for (key, glyphname) in &data {
        println!("{}: {}", key, glyphname);
        let c: char = char::from_u32(*key).unwrap();
        let text: &str = &c.to_string();

        let segments = fcx.get_music_string_segments(SCALE, text);
        dbg!(&segments);
        // fs::write("libs/fonts/examples/export.rs", format!("pub const {}: &[PathSegment] = &{:?};", glyphname, segments[0])).unwrap();

        let line = format!("pub const GLYPH_{}: &[PathSegment] = &{:?};", glyphname, segments[0]);
        lines.push(line);
    }

    let code = lines.join("\n\n");
    // fs::write("libs/fonts/examples/export.rs", code).unwrap();

    fs::write("libs/render/src/music_glyphs.rs", code).unwrap();

    // let text = "A";

    // let mut chars: Chars = text.chars();
    // let x: char = chars.next().unwrap();
    // let glyphid: GlyphId = x.into_glyph_id(&fcx.music_font.borrow());

    // let char = 65u8 as usize;

    // let g = fcx.music_font.borrow().glyph(c).scaled(Scale::uniform(50.0));
    // dbg!(&g);

    // let segments = fcx.get_music_string_segments(50.0, text);
    // dbg!(&segments);
    // fs::write("libs/fonts/examples/export.rs", format!("pub const {}: &[PathSegment] = &{:?};", glyphname, segments[0])).unwrap();

    // let items = fcx.get_music_string_items(50.0, text);
    // let svg = SvgBuilder::new().build(items, None);
    // fs::write("libs/fonts/examples/ex-fonts-export.svg", svg).unwrap();
}
