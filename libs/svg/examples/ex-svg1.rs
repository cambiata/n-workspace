use graphics::{
    color::Color,
    fill::Fill,
    graphicitem::{GraphicItem, GraphicItems},
    stroke::Stroke,
};
use std::fs;
use svg::svg_renderer::SvgBuilder;

fn main() {
    let mut items = GraphicItems::new();
    items.push(GraphicItem::Rect(0., 0., 20., 20., Stroke::None, Fill::Solid(Color::Tomato), None));
    items.push(GraphicItem::Line(0., 0., 50., 15., Stroke::Solid(1., Color::DodgerBlue), None));
    let mut svg = SvgBuilder::new();
    let svg_string = svg.build(items, None);

    let filename = "./libs/svg/examples/ex_svg1.svg";
    fs::write(filename, svg_string).unwrap();
    println!("SVG file created successfully at {filename}");
}
