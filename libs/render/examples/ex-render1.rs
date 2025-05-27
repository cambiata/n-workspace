use graphics::color::Color;
use grid::{gridcontext::GridContext, griditem::GridItemType};
use render::gridrender::render_gridcontext_with_color;

fn main() {
    let items = vec![
        vec![
            GridItemType::Rectangles(vec![(0.0, 0.0, 5.0, 5.0)], vec![Color::Blue]),
            GridItemType::Rectangles(vec![(0.0, 3.0, 20.0, 5.0)], vec![Color::Tomato]),
            GridItemType::Rectangles(vec![(0.0, 0.0, 5.0, 5.0)], vec![Color::Orange]),
        ],
        vec![
            GridItemType::Rectangles(vec![(0.0, 0.0, 10.0, 5.0), (0.0, 5.0, 10.0, 5.0)], vec![Color::Green, Color::Lime]),
            // GridItemType::Empty,
            GridItemType::Rectangles(vec![(0.0, 3.0, 5.0, 5.0)], vec![Color::Purple]),
            GridItemType::Rectangles(vec![(-10.0, 0.0, 15.0, 5.0)], vec![Color::DodgerBlue]),
        ],
    ];

    let cx: &'static GridContext<Color> = GridContext::<Color>::new();
    cx.add_items(items);
    cx.calculate_minimal_col_spacing();
    render_gridcontext_with_color(cx);
}
