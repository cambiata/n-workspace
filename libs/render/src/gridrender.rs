use std::fs;

use graphics::{
    color::Color,
    fill::Fill,
    graphicitem::{GraphicItem, GraphicItems},
    stroke::Stroke,
};
use grid::{gridcontext::GridContext, griditem::GridItemType};
use svg::svg_renderer::SvgBuilder;

#[allow(dead_code)]
pub fn render_gridcontext_with_color(cx: &'static GridContext<Color>) {
    let mut graphic_items = GraphicItems::new();

    let cx_rows = &cx.rows.borrow();
    let cx_cols_overlaps = &cx.cols_overlaps.borrow();

    let mut move_y = 0.0;
    for row in cx_rows.iter() {
        let mut move_x = 0.0;
        for (colidx, item_id) in row.item_ids.iter().enumerate() {
            move_x += cx_cols_overlaps[colidx];
            let item = &cx.items.borrow()[*item_id];
            match item.gitype {
                GridItemType::Rectangles(ref rects, ref colors) => {
                    for (rect, color) in rects.iter().zip(colors.iter()) {
                        graphic_items.push(GraphicItem::Rect(rect.0 + move_x, rect.1 + move_y, rect.2, rect.3, Stroke::None, Fill::Solid(*color), None));
                    }
                }
                GridItemType::Empty => {
                    println!("Rendering empty item");
                    graphic_items.push(GraphicItem::Rect(move_x - 1., move_y, 2., 2., Stroke::None, Fill::Solid(Color::Orange), None));
                }
            }
        }
        move_y += 20.0;
    }

    let mut move_x = 0.0;
    for (_colidx, overlap) in cx_cols_overlaps.iter().enumerate() {
        move_x = move_x + *overlap;
        // dbg!(overlap, move_x);

        graphic_items.push(GraphicItem::Line(move_x, 0.0, move_x, move_y, Stroke::Solid(0.5, Color::RGBA(0, 0, 0, 0.5)), None));
    }

    // let graphic_items = items_scale(graphic_items, 3.0, 3.0);

    let svg = SvgBuilder::new();
    let svg_string = svg.build(graphic_items, None);
    fs::write("out/ex-render1.svg", svg_string).unwrap();
}
