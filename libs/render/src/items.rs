use core::{accidental::Accidental, clef::ClefSignature, head::HeadType};

use graphics::{color::Color, fill::Fill, graphicitem::GraphicItem, rectangle::Rectangle, stroke::Stroke};
use score::{
    constants::{SPACE, SPACE2, SPACE3, SPACE_HALF},
    glyphitem::GlyphItem,
};

use crate::music_glyphs::{
    GLYPH_ACCIDENTAL_FLAT, GLYPH_ACCIDENTAL_NATURAL, GLYPH_ACCIDENTAL_SHARP, GLYPH_CLEF_BASS, GLYPH_CLEF_TREBLE, GLYPH_FIVELINES, GLYPH_NOTEHEAD_BLACK, GLYPH_NOTEHEAD_WHITE, GLYPH_NOTEHEAD_WHOLE,
};

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
            let path = match _htype {
                HeadType::Whole => GLYPH_NOTEHEAD_WHOLE,
                HeadType::Brevis => GLYPH_NOTEHEAD_WHOLE,
                HeadType::White => GLYPH_NOTEHEAD_WHITE,
                _ => GLYPH_NOTEHEAD_BLACK,
            };

            // graphic_items.push(GraphicItem::Rect(rect.0 + movex, rect.1 + movey, rect.2, rect.3, Stroke::None, Fill::Solid(Color::LightGray), None));

            graphic_items.push(GraphicItem::Path(
                path.to_vec(),
                rect.0 + movex,
                rect.1 + movey + y_zero - SPACE3 - SPACE_HALF,
                Stroke::None,
                Fill::Solid(Color::Black),
                None,
            ));
        }
        GlyphItem::Accidental(_atype) => {
            let path = match _atype {
                Accidental::Flat => GLYPH_ACCIDENTAL_FLAT,
                Accidental::Natural => GLYPH_ACCIDENTAL_NATURAL,
                _ => GLYPH_ACCIDENTAL_SHARP,
            };

            // graphic_items.push(GraphicItem::Rect(rect.0 + movex, rect.1 + movey, rect.2, rect.3, Stroke::None, Fill::Solid(Color::LightGray), None));

            graphic_items.push(GraphicItem::Path(
                path.to_vec(),
                rect.0 + movex,
                rect.1 + movey + y_zero - SPACE2 - SPACE_HALF,
                Stroke::None,
                Fill::Solid(Color::Black),
                None,
            ));
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

            let y = match _ctype {
                ClefSignature::Bass => -SPACE3,
                _ => -SPACE,
            };

            graphic_items.push(GraphicItem::Path(
                curve.to_vec(),
                rect.0 + movex,
                rect.1 + movey + y_zero + y,
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
