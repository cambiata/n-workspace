use graphics::{color::Color, fill::Fill, graphicitem::GraphicItem, rectangle::Rectangle, stroke::Stroke};
use score::glyphitem::GlyphItem;

pub fn get_graphic_items_from_glyph(movex: f32, movey: f32, rect: &Rectangle, glyph: &GlyphItem) -> Vec<GraphicItem> {
    let mut graphic_items = Vec::new();
    match glyph {
        GlyphItem::XRect(color) => {
            graphic_items.push(GraphicItem::Rect(rect.0 + movex, rect.1 + movey, rect.2, rect.3, Stroke::None, Fill::Solid(*color), None));
        }
        GlyphItem::Barline(_btype) => {
            graphic_items.push(GraphicItem::Rect(rect.0 + movex, rect.1 + movey, rect.2, rect.3, Stroke::None, Fill::Solid(Color::Black), None));
        }
        GlyphItem::Notehead(_htype, _hvariant) => {
            graphic_items.push(GraphicItem::Rect(rect.0 + movex, rect.1 + movey, rect.2, rect.3, Stroke::None, Fill::Solid(Color::DodgerBlue), None));
        }
        GlyphItem::Accidental(_atype) => {
            graphic_items.push(GraphicItem::Rect(rect.0 + movex, rect.1 + movey, rect.2, rect.3, Stroke::None, Fill::Solid(Color::Purple), None));
        }
        _ => {
            graphic_items.push(GraphicItem::Rect(rect.0 + movex, rect.1 + movey, rect.2, rect.3, Stroke::None, Fill::Solid(Color::LightGray), None));
        }
    }

    graphic_items
}
