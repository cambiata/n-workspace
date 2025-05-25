use crate::fill::Fill;
use crate::path::{path_bounding_box, path_move, PathSegment};
use crate::stroke::Stroke;

#[derive(Debug, Clone)]
pub enum GraphicItem {
    Line(f32, f32, f32, f32, Stroke, Option<String>),
    Rect(f32, f32, f32, f32, Stroke, Fill, Option<String>),
    Ellipse(f32, f32, f32, f32, Stroke, Fill, Option<String>),
    Path(Vec<PathSegment>, f32, f32, Stroke, Fill, Option<String>),
    Text(f32, f32, String, Option<String>),
}

pub type GraphicItems = Vec<GraphicItem>;

pub fn items_scale(items: GraphicItems, scale_x: f32, scale_y: f32) -> GraphicItems {
    items
        .iter()
        .map(|item| match item {
            GraphicItem::Line(x1, y1, x2, y2, stroke, id) => GraphicItem::Line(x1 * scale_x, y1 * scale_y, x2 * scale_x, y2 * scale_y, stroke.clone(), id.clone()),
            GraphicItem::Rect(x, y, w, h, stroke, fill, id) => GraphicItem::Rect(x * scale_x, y * scale_y, w * scale_x, h * scale_y, stroke.clone(), fill.clone(), id.clone()),
            GraphicItem::Ellipse(x, y, w, h, stroke, fill, id) => GraphicItem::Ellipse(x * scale_x, y * scale_y, w * scale_x, h * scale_y, stroke.clone(), fill.clone(), id.clone()),
            GraphicItem::Path(segments, x, y, stroke, fill, id) => {
                let new_segments = segments
                    .iter()
                    .map(|segment| match segment {
                        PathSegment::M(x, y) => PathSegment::M(x * scale_x, y * scale_y),
                        PathSegment::L(x, y) => PathSegment::L(x * scale_x, y * scale_y),
                        PathSegment::Q(x1, y1, x2, y2) => PathSegment::Q(x1 * scale_x, y1 * scale_y, x2 * scale_x, y2 * scale_y),
                        PathSegment::C(x1, y1, x2, y2, x3, y3) => PathSegment::C(x1 * scale_x, y1 * scale_y, x2 * scale_x, y2 * scale_y, x3 * scale_x, y3 * scale_y),
                        PathSegment::Z => PathSegment::Z,
                    })
                    .collect();
                GraphicItem::Path(new_segments, x * scale_x, y * scale_y, stroke.clone(), fill.clone(), id.clone())
            }
            GraphicItem::Text(x, y, text, id) => GraphicItem::Text(x * scale_x, y * scale_y, text.clone(), id.clone()),
        })
        .collect()
}

pub fn items_move(items: GraphicItems, move_x: f32, move_y: f32) -> GraphicItems {
    items
        .iter()
        .map(|item| match item {
            GraphicItem::Line(x1, y1, x2, y2, stroke, id) => GraphicItem::Line(x1 + move_x, y1 + move_y, x2 + move_x, y2 + move_y, stroke.clone(), id.clone()),
            GraphicItem::Rect(x, y, w, h, stroke, fill, id) => GraphicItem::Rect(x + move_x, y + move_y, *w, *h, stroke.clone(), fill.clone(), id.clone()),
            GraphicItem::Ellipse(x, y, w, h, stroke, fill, id) => GraphicItem::Ellipse(x + move_x, y + move_y, *w, *h, stroke.clone(), fill.clone(), id.clone()),
            GraphicItem::Path(segments, x, y, stroke, fill, id) => {
                let new_segments = path_move(segments.clone(), move_x, move_y);
                GraphicItem::Path(new_segments, *x, *y, stroke.clone(), fill.clone(), id.clone())
            }
            GraphicItem::Text(x, y, text, id) => GraphicItem::Text(x + move_x, y + move_y, text.clone(), id.clone()),
        })
        .collect()
}

pub fn items_bounding_box(items: &GraphicItems) -> (f32, f32, f32, f32) {
    let mut min_x: f32 = f32::MAX;
    let mut min_y: f32 = f32::MAX;
    let mut max_x: f32 = f32::MIN;
    let mut max_y: f32 = f32::MIN;

    items.iter().for_each(|item| match item {
        GraphicItem::Line(x1, y1, x2, y2, _, _) => {
            min_x = min_x.min(*x1).min(*x2);
            min_y = min_y.min(*y1).min(*y2);
            max_x = max_x.max(*x1).max(*x2);
            max_y = max_y.max(*y1).max(*y2);
        }
        GraphicItem::Rect(x, y, w, h, _, _, _) => {
            min_x = min_x.min(*x);
            min_y = min_y.min(*y);
            max_x = max_x.max(*x + w);
            max_y = max_y.max(*y + h);
        }
        GraphicItem::Ellipse(x, y, w, h, _, _, _) => {
            min_x = min_x.min(*x - w);
            min_y = min_y.min(*y - h);
            max_x = max_x.max(*x + w);
            max_y = max_y.max(*y + h);
        }
        GraphicItem::Path(segments, x, y, _, _, _) => {
            let (min_x_, min_y_, max_x_, max_y_) = path_bounding_box(segments.clone());
            // dbg!(min_x_, min_y_, max_x_, max_y_, x, y);

            min_x = min_x.min(min_x_ + x);
            min_y = min_y.min(min_y_ + y);
            max_x = max_x.max(max_x_ + x);
            max_y = max_y.max(max_y_ + y);
            // min_x = min_x.min(min_x_ + x);
            // min_y = min_y.min(min_y_ + y);
            // max_x = max_x.max(max_x_ - x);
            // max_y = max_y.max(max_y_ - y);
        } //-----------
        _ => {} //

                // GraphicItem::Text(x, y, text, font, _) => {
                //     let (w, h) = text_size(text.clone(), font.clone());
                //     min_x = min_x.min(x);
                //     min_y = min_y.min(y);
                //     max_x = max_x.max(x + w);
                //     max_y = max_y.max(y + h);
                // }
    });

    (min_x, min_y, max_x, max_y)
}
