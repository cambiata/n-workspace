use std::fs;

use fonts::pathbuilder::PathBuilder;
use graphics::{color::Color, fill::Fill, graphicitem::GraphicItem, stroke::Stroke};
use image::{DynamicImage, Rgba};
use rusttype::{point, Font, Scale};
use svg::svg_renderer::SvgBuilder;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font_data = include_bytes!("../assets/OpenSans-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    let scale = Scale::uniform(32.0);
    let text = "Hej";
    let colour = (255, 255, 255);
    let v_metrics = font.v_metrics(scale);

    // layout the glyphs
    let glyphs: Vec<_> = font.layout(text, scale, point(0., v_metrics.ascent)).collect();

    // work out the layout size
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs.first().map(|g| g.pixel_bounding_box().unwrap().min.x).unwrap();
        let max_x = glyphs.last().map(|g| g.pixel_bounding_box().unwrap().max.x).unwrap();
        (max_x - min_x) as u32
    };

    dbg!(glyphs_height, glyphs_width);

    //---------------------------
    let mut image = DynamicImage::new_rgba8(glyphs_width, glyphs_height).to_rgba8();

    for glyph in &glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                image.put_pixel(
                    // Offset the position by the glyph bounding box
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    // Turn the coverage into an alpha value
                    Rgba([colour.0, colour.1, colour.2, (v * 255.0) as u8]),
                )
            });
        }
    }

    image.save("libs/fonts/examples/image_example.png").unwrap();

    //---------------------------
    let mut items = Vec::new();

    for glyph in &glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            items.push(GraphicItem::Rect(
                bb.min.x as f32,
                bb.min.y as f32,
                (bb.max.x - bb.min.x) as f32,
                (bb.max.y - bb.min.y) as f32,
                Stroke::None,
                Fill::Solid(Color::LightGray),
                None,
            ));

            let mut builder = PathBuilder { x: 0., y: 0., segments: Vec::new() };
            glyph.build_outline(&mut builder);
            let segments = builder.get_segments();

            items.push(GraphicItem::Path(
                segments,
                bb.min.x as f32,
                bb.min.y as f32,
                Stroke::Solid(0.1, Color::Black),
                Fill::Solid(Color::DodgerBlue),
                None,
            ));
        }
    }
    let svg = SvgBuilder::new().build(items, None);
    fs::write("libs/fonts/examples/ex-fonts2.svg", svg).unwrap();

    Ok(())
}
