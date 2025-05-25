pub mod svg_renderer;

#[cfg(test)]
mod tests {
    use graphics::{
        color::Color,
        fill::Fill,
        graphicitem::{GraphicItem, GraphicItems},
        stroke::Stroke,
    };
    use std::fs;

    use crate::svg_renderer::SvgBuilder;

    #[test]
    fn test_svg() {
        let mut items = GraphicItems::new();
        items.push(GraphicItem::Rect(0., 0., 20., 20., Stroke::None, Fill::Solid(Color::Tomato), None));
        items.push(GraphicItem::Line(0., 0., 50., 15., Stroke::Solid(1., Color::DodgerBlue), None));
        let svg = SvgBuilder::new();
        let svg_string = svg.build(items, None);

        fs::write("out/test.svg", svg_string).unwrap();
    }
}
