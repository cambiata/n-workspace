use crate::items::get_graphic_items_from_glyph;
use graphics::{
    color::Color,
    fill::Fill,
    graphicitem::{GraphicItem, GraphicItems},
    stroke::Stroke,
};
use grid::{gridcontext::GridContext, griditem::GridItemType};
use score::{constants::SPACE, glyphitem::GlyphItem};
use svg::svg_renderer::SvgBuilder;

pub struct Render;
impl Render {
    pub fn render_notelines(gcx: &'static GridContext<GlyphItem>) -> GraphicItems {
        let mut graphic_items = GraphicItems::new();
        let cx_rows = &gcx.rows.borrow();
        let cols_widths = &gcx.cols_widths.borrow();
        let row_heights = &gcx.rows_heights.borrow();

        let mut move_y = 0.0;
        for (row_idx, _row) in cx_rows.iter().enumerate() {
            move_y += row_heights[row_idx];
            let mut left_x = cols_widths[0];
            for (_colidx, widths) in cols_widths.windows(2).enumerate() {
                let width = widths[1];
                for i in -2..=2 {
                    let line_y = move_y + SPACE * i as f32;
                    graphic_items.push(GraphicItem::Line(left_x, line_y, left_x + width, line_y, Stroke::Solid(1.0, Color::Black), None));
                }
                left_x += width;
            }
        }
        graphic_items
    }

    pub fn render_music_glyphitems(gcx: &'static GridContext<GlyphItem>) -> GraphicItems {
        let mut graphic_items = GraphicItems::new();
        let cx_rows = &gcx.rows.borrow();
        let cx_cols_overlaps = &gcx.cols_widths.borrow();
        let row_heights = &gcx.rows_heights.borrow();

        let mut move_y = 0.0;

        for (row_idx, row) in cx_rows.iter().enumerate() {
            move_y += row_heights[row_idx];

            let mut move_x = 0.0;
            for (colidx, item_id) in row.item_ids.iter().enumerate() {
                move_x += cx_cols_overlaps[colidx];
                let item = &gcx.items.borrow()[*item_id];
                match item.gitype {
                    GridItemType::Rectangles(ref glyph_items) => {
                        for (rect, glyph_item) in glyph_items.iter() {
                            graphic_items.extend(get_graphic_items_from_glyph(move_x, move_y, &rect, &glyph_item));
                        }
                    }
                    GridItemType::Empty => {
                        graphic_items.push(GraphicItem::Rect(move_x, move_y, 1., 1., Stroke::None, Fill::Solid(Color::Orange), None));
                    }
                }
            }
        }

        // let mut move_x = 0.0;
        // for (_colidx, overlap) in cx_cols_overlaps.iter().enumerate() {
        //     move_x = move_x + *overlap;
        //     graphic_items.push(GraphicItem::Line(move_x - 0.1, 0.0, move_x, move_y, Stroke::Solid(0.2, Color::RGBA(0, 0, 0, 0.2)), None));
        // }
        graphic_items
    }

    #[allow(dead_code)]
    pub fn render_gridcontext_with_color(gcx: &'static GridContext<Color>) -> String {
        let mut graphic_items = GraphicItems::new();
        let cx_rows = &gcx.rows.borrow();
        let cx_cols_overlaps = &gcx.cols_widths.borrow();

        let mut move_y = 0.0;

        for row in cx_rows.iter() {
            let mut move_x = 0.0;
            for (colidx, item_id) in row.item_ids.iter().enumerate() {
                move_x += cx_cols_overlaps[colidx];
                let item = &gcx.items.borrow()[*item_id];
                match item.gitype {
                    GridItemType::Rectangles(ref items) => {
                        for (rect, color) in items.iter() {
                            graphic_items.push(GraphicItem::Rect(rect.0 + move_x, rect.1 + move_y, rect.2, rect.3, Stroke::None, Fill::Solid(*color), None));
                        }
                    }
                    GridItemType::Empty => {
                        // println!("Rendering empty item");
                        graphic_items.push(GraphicItem::Rect(move_x, move_y, 1., 1., Stroke::None, Fill::Solid(Color::Orange), None));
                    }
                }
            }
            move_y += 10.0;
        }

        let mut move_x = 0.0;
        for (_colidx, overlap) in cx_cols_overlaps.iter().enumerate() {
            move_x = move_x + *overlap;
            graphic_items.push(GraphicItem::Line(move_x - 0.1, 0.0, move_x, move_y, Stroke::Solid(0.2, Color::RGBA(0, 0, 0, 0.2)), None));
        }

        // let graphic_items = items_scale(graphic_items, 3.0, 3.0);

        let mut svg = SvgBuilder::new();
        let svg_string = svg.build(graphic_items, None);

        svg_string
    }
}
