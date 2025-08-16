use graphics::{color::Color, fill::Fill, graphicitem::GraphicItem, path::PathSegment, stroke::Stroke};
use rusttype::{point, Font, PositionedGlyph, Scale};

use crate::pathbuilder::PathBuilder;

// pub fn get_items(font: &Font, scale: f32, text: &str) -> Vec<GraphicItem> {
//     let scale = Scale::uniform(scale);
//     let v_metrics = font.v_metrics(scale);
//     let glyphs: Vec<PositionedGlyph<'_>> = font.layout(text, scale, point(0., v_metrics.ascent)).collect();
//     let mut items = Vec::new();
//     for glyph in &glyphs {
//         if let Some(bb) = glyph.pixel_bounding_box() {
//             let mut builder = PathBuilder { x: 0., y: 0., segments: Vec::new() };
//             glyph.build_outline(&mut builder);
//             let segments = builder.get_segments();
//             items.push(GraphicItem::Path(
//                 segments,
//                 bb.min.x as f32,
//                 bb.min.y as f32,
//                 Stroke::Solid(0.1, Color::Black),
//                 Fill::Solid(Color::DodgerBlue),
//                 None,
//             ));
//         }
//     }
//     items
// }

pub fn get_items(font: &Font, scale: f32, text: &str) -> Vec<GraphicItem> {
    let segments = get_segments(font, scale, text);
    let mut items = Vec::new();
    for segment in segments {
        items.push(GraphicItem::Path(segment, 0.0, 0.0, Stroke::None, Fill::Solid(Color::Red), None));
    }
    items
}

pub fn get_segments(font: &Font, scale: f32, text: &str) -> Vec<Vec<PathSegment>> {
    let scale = Scale::uniform(scale);
    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<PositionedGlyph<'_>> = font.layout(text, scale, point(0., v_metrics.ascent)).collect();
    let mut segments2 = Vec::new();
    for glyph in &glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            let mut builder = PathBuilder {
                x: bb.min.x as f32,
                y: bb.min.y as f32,
                segments: Vec::new(),
            };
            glyph.build_outline(&mut builder);
            let segments = builder.get_segments();
            segments2.push(segments);
        }
    }
    dbg!(&segments2);
    segments2
}

pub fn get_dimensions(font: &Font, scale: f32, text: &str) -> (f32, f32) {
    let scale = Scale::uniform(scale);
    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<PositionedGlyph<'_>> = font.layout(text, scale, point(0., v_metrics.ascent)).collect();
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as f32;
    let glyphs_width = {
        let min_x = glyphs.first().map(|g| g.pixel_bounding_box().unwrap().min.x).unwrap_or(0) as f32;
        let max_x = glyphs.last().map(|g| g.pixel_bounding_box().unwrap().max.x).unwrap_or(0) as f32;
        (max_x - min_x)
    };
    (glyphs_width, glyphs_height)
}
