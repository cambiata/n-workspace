use std::fs;

use fonts::{fontcontext::FontContext, pathbuilder::PathBuilder};
use graphics::{
    color::Color,
    fill::Fill,
    graphicitem::{GraphicItem, GraphicItems},
    stroke::Stroke,
};
use rusttype::{point, Font, Point, Rect, Scale};
use svg::builder::SvgBuilder;

// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-fonts1"
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fcx = FontContext::new();
    dbg!(&fcx);
    let font = fcx.sansserif_font.borrow();

    let text = "Hej hopp";
    let scale = Scale::uniform(50.0);
    let glyphs: Vec<_> = font.layout(text, scale, point(0., 0.)).collect();
    let v_metrics = font.v_metrics(scale);
    dbg!(&v_metrics);
    for glyph in &glyphs {
        dbg!(&glyph);
    }

    // work out the layout size
    let _glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let _glyphs_width = {
        let min_x = glyphs.first().map(|g| g.pixel_bounding_box().unwrap().min.x).unwrap();
        let max_x = glyphs.last().map(|g| g.pixel_bounding_box().unwrap().max.x).unwrap();
        (max_x - min_x) as u32
    };

    dbg!(_glyphs_height, _glyphs_width);

    let mut x = 0.;
    let mut all_segments = Vec::new();

    for glyph in glyphs {
        println!("glyph id: {}", glyph.id().0);
        let bounding_box = match glyph.unpositioned().exact_bounding_box() {
            Some(bounding_box) => bounding_box,
            None => Rect {
                min: Point { x: scale.x / 5., y: 0. },
                max: Point { x: scale.x / 5., y: 0. },
            },
        };
        x = glyph.position().x;
        let mut builder = PathBuilder {
            x,
            y: v_metrics.ascent + bounding_box.min.y,
            segments: Vec::new(),
        };

        glyph.build_outline(&mut builder);
        let segments = builder.get_segments();
        all_segments.push(segments);
        // x += bounding_box.width();
    }

    let mut items: GraphicItems = Vec::new();
    for segments in all_segments {
        items.push(GraphicItem::Path(
            segments,
            0.,
            0.,
            Stroke::Solid(0.1, Color::Black),
            Fill::Solid(Color::Tomato),
            graphics::graphicitem::PathCache::NoCache,
        ));
    }

    let svg = SvgBuilder::new().build(items, None);
    fs::write("libs/fonts/examples/ex-fonts1.svg", svg).unwrap();

    Ok(())
}
/*
pub fn mainx() -> Result<(), Box<dyn std::error::Error>> {
    // let font_data = include_bytes!("../assets/MTF-Cadence-Fin.ttf");
    let font_data = include_bytes!("../assets/OpenSans-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");
    let scale = Scale::uniform(50.0);
    let v_metrics = font.v_metrics(scale);

    // let text = "&-";
    let text = "ABCabcåäö123";
    let glyphs: Vec<_> = font.layout(text, scale, point(0., 0.)).collect();

    // work out the layout size
    let _glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let _glyphs_width = {
        let min_x = glyphs.first().map(|g| g.pixel_bounding_box().unwrap().min.x).unwrap();
        let max_x = glyphs.last().map(|g| g.pixel_bounding_box().unwrap().max.x).unwrap();
        (max_x - min_x) as u32
    };
    // dbg!(glyphs_height, glyphs_width);

    let mut x = 0.;

    let mut all_segments = Vec::new();

    for glyph in glyphs {
        println!("glyph id: {}", glyph.id().0);
        let bounding_box = match glyph.unpositioned().exact_bounding_box() {
            Some(bounding_box) => bounding_box,
            None => Rect {
                min: Point { x: scale.x / 5., y: 0. },
                max: Point { x: scale.x / 5., y: 0. },
            },
        };
        x += bounding_box.min.x;
        let mut builder = PathBuilder {
            x,
            y: v_metrics.ascent + bounding_box.min.y,
            segments: Vec::new(),
        };

        glyph.build_outline(&mut builder);
        let segments = builder.get_segments();
        all_segments.push(segments);
        x += bounding_box.width();
    }

    let mut items: GraphicItems = Vec::new();
    for segments in all_segments {
        items.push(GraphicItem::Path(segments, 0., 0., Stroke::Solid(0.1, Color::Black), Fill::Solid(Color::Tomato), None));
    }

    let svg = SvgBuilder::new().build(items, None);
    fs::write("out/fonts.svg", svg).unwrap();

    Ok(())
}
    */
