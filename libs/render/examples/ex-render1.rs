// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-render1"

use std::fs;

use graphics::color::Color;
use grid::{gridcontext::GridContext, griditem::GridItemType};
use render::gridrender::render_gridcontext_with_color;

fn main() {
    let items = vec![
        vec![
            GridItemType::Rectangles(vec![((0.0, 0.0, 5.0, 5.0), Color::Red)]),
            GridItemType::Empty,
            GridItemType::Rectangles(vec![((0.0, 0.0, 5.0, 5.0), Color::Orange)]),
        ],
        vec![
            GridItemType::Empty,
            GridItemType::Rectangles(vec![((0.0, 0.0, 6.0, 5.0), Color::Green)]),
            GridItemType::Rectangles(vec![((-4.0, 0.0, 8.0, 5.0), Color::Tomato)]),
        ],
        vec![
            GridItemType::Rectangles(vec![((0.0, 0.0, 1.0, 5.0), Color::Green)]),
            GridItemType::Rectangles(vec![((0.0, 0.0, 2.0, 5.0), Color::Blue)]),
            GridItemType::Rectangles(vec![((0.0, 0.0, 5.0, 5.0), Color::Purple)]),
        ],
    ];

    let cx: &'static GridContext<Color> = GridContext::<Color>::new();
    cx.add_items(items);
    cx.calculate_minimal_col_spacing();
    cx.set_durations(vec![0, 8, 10]); //

    fs::write("libs/render/examples/ex-render1.svg", render_gridcontext_with_color(cx)).unwrap();
}
