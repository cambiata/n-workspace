use std::fs;

use graphics::color::Color;
use grid::{gridcontext::GridContext, griditem::GridItemType};
use render::gridrender::render_gridcontext_with_color;

fn main() {
    let items = vec![
        vec![
            GridItemType::Rectangles(vec![(0.0, 0.0, 5.0, 5.0)], vec![Color::Red]),
            GridItemType::Empty,
            GridItemType::Rectangles(vec![(0.0, 0.0, 5.0, 5.0)], vec![Color::Orange]),
        ],
        vec![
            GridItemType::Empty,
            GridItemType::Rectangles(vec![(0.0, 0.0, 6.0, 5.0)], vec![Color::Green]),
            GridItemType::Rectangles(vec![(-4.0, 0.0, 8.0, 5.0)], vec![Color::Tomato]),
        ],
        vec![
            GridItemType::Rectangles(vec![(0.0, 0.0, 1.0, 5.0)], vec![Color::Green]),
            GridItemType::Rectangles(vec![(0.0, 0.0, 2.0, 5.0)], vec![Color::Blue]),
            GridItemType::Rectangles(vec![(0.0, 0.0, 5.0, 5.0)], vec![Color::Purple]),
        ],
        // vec![
        //     GridItemType::Rectangles(vec![(0.0, 0.0, 10.0, 5.0), (0.0, 5.0, 10.0, 5.0)], vec![Color::Green, Color::Lime]),
        //     // GridItemType::Empty,
        //     GridItemType::Rectangles(vec![(0.0, 3.0, 5.0, 5.0)], vec![Color::Purple]),
        //     GridItemType::Rectangles(vec![(-10.0, 0.0, 15.0, 5.0)], vec![Color::DodgerBlue]),
        // ],
    ];

    // let items = vec![vec![
    //     GridItemType::Rectangles(vec![(0.0, 0.0, 5.0, 5.0)], vec![Color::Purple]),
    //     GridItemType::Empty,
    //     GridItemType::Rectangles(vec![(0.0, 0.0, 5.0, 5.0)], vec![Color::Purple]),
    // ]];

    let cx: &'static GridContext<Color> = GridContext::<Color>::new();
    cx.add_items(items);
    cx.calculate_minimal_col_spacing();
    cx.set_durations(vec![0, 8, 10]); //

    fs::write("out/ex-render1.svg", render_gridcontext_with_color(cx)).unwrap();
}
