use graphics::color::Color;
use grid::{gridcontext::GridContext, griditem::GridItemType};
use render::gridrender::render_gridcontext_with_color;

fn main() {
    let items = vec![
        vec![
            GridItemType::Rectangles(vec![(0.0, 0.0, 5.0, 10.0)], vec![Color::Blue]),
            // GridItemType::Empty,
            GridItemType::Rectangles(vec![(0.0, 0.0, 20.0, 10.0)], vec![Color::Red]),
            GridItemType::Rectangles(vec![(0.0, 0.0, 15.0, 10.0)], vec![Color::Orange]),
        ],
        vec![
            GridItemType::Rectangles(vec![(0.0, 0.0, 10.0, 10.0)], vec![Color::Green]),
            GridItemType::Empty,
            // GridItemType::Rectangles(vec![(5.0, 0.0, 5.0, 10.0)], vec![Color::Gray]),
            GridItemType::Rectangles(vec![(-5.0, 0.0, 15.0, 10.0)], vec![Color::Purple]),
        ],
    ];

    let cx: &'static GridContext<Color> = GridContext::<Color>::new();
    cx.add_items(items);
    cx.calculate_minimal_col_spacing();
    render_gridcontext_with_color(cx);
}
