use core::clef::ClefSignature;

use graphics::{color::Color, fill::Fill, graphicitem::GraphicItem, rectangle::Rectangle, stroke::Stroke};
use score::{
    constants::{SPACE, SPACE2},
    glyphitem::GlyphItem,
};

use crate::music_glyphs::{GLYPH_CLEF_BASS, GLYPH_CLEF_TREBLE, GLYPH_FIVELINES};

pub fn get_graphic_items_from_glyph(movex: f32, movey: f32, rect: &Rectangle, glyph: &GlyphItem) -> Vec<GraphicItem> {
    let y_zero = -SPACE2 * 1.0;

    let mut graphic_items = Vec::new();

    match glyph {
        GlyphItem::XRect(color) => {
            graphic_items.push(GraphicItem::Rect(rect.0 + movex, rect.1 + movey, rect.2, rect.3, Stroke::None, Fill::Solid(*color), None));
        }
        GlyphItem::Barline(_btype) => {
            graphic_items.push(GraphicItem::Rect(rect.0 + movex, rect.1 + movey, rect.2, rect.3, Stroke::None, Fill::Solid(Color::Black), None));
        }
        GlyphItem::Notehead(_htype, _hvariant) => {
            graphic_items.push(GraphicItem::Rect(rect.0 + movex, rect.1 + movey, rect.2, rect.3, Stroke::None, Fill::Solid(Color::Lime), None));
        }
        GlyphItem::Accidental(_atype) => {
            graphic_items.push(GraphicItem::Rect(rect.0 + movex, rect.1 + movey, rect.2, rect.3, Stroke::None, Fill::Solid(Color::Purple), None));
        }

        GlyphItem::Clef(_ctype) => {
            graphic_items.push(GraphicItem::Rect(rect.0 + movex, rect.1 + movey, rect.2, rect.3, Stroke::None, Fill::Solid(Color::LightGray), None));
            graphic_items.push(GraphicItem::Path(
                GLYPH_FIVELINES.to_vec(),
                rect.0 + movex,
                rect.1 + movey + y_zero,
                Stroke::None,
                Fill::Solid(Color::Red),
                None,
            ));

            let curve = match _ctype {
                ClefSignature::Treble => GLYPH_CLEF_TREBLE,
                ClefSignature::Bass => GLYPH_CLEF_BASS,
                _ => GLYPH_CLEF_TREBLE,
            };

            graphic_items.push(GraphicItem::Path(
                curve.to_vec(),
                rect.0 + movex,
                rect.1 + movey + y_zero - SPACE,
                Stroke::None,
                Fill::Solid(Color::Black),
                None,
            ));
        }

        _ => {
            graphic_items.push(GraphicItem::Rect(rect.0 + movex, rect.1 + movey, rect.2, rect.3, Stroke::None, Fill::Solid(Color::LightGray), None));
        }
    }

    graphic_items
}
