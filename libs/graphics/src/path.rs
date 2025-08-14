#[derive(Debug, Clone)]
pub enum PathSegment {
    M(f32, f32),
    L(f32, f32),
    Q(f32, f32, f32, f32),
    C(f32, f32, f32, f32, f32, f32),
    Z,
}

pub type PathSegments = Vec<PathSegment>;

pub fn path_move(segments: PathSegments, move_x: f32, move_y: f32) -> PathSegments {
    // dbg!(&move_x, &move_y);
    segments
        .iter()
        .map(|segment| match segment {
            PathSegment::M(x, y) => PathSegment::M(x + move_x, y + move_y),
            PathSegment::L(x, y) => PathSegment::L(x + move_x, y + move_y),
            PathSegment::Q(x1, y1, x2, y2) => PathSegment::Q(x1 + move_x, y1 + move_y, x2 + move_x, y2 + move_y),
            PathSegment::C(x1, y1, x2, y2, x3, y3) => PathSegment::C(x1 + move_x, y1 + move_y, x2 + move_x, y2 + move_y, x3 + move_x, y3 + move_y),
            PathSegment::Z => PathSegment::Z,
        })
        .collect()
}

pub fn path_scale(segments: PathSegments, scale_x: f32, scale_y: f32) -> PathSegments {
    segments
        .iter()
        .map(|segment| match segment {
            PathSegment::M(x, y) => PathSegment::M(x * scale_x, y * scale_y),
            PathSegment::L(x, y) => PathSegment::L(x * scale_x, y * scale_y),
            PathSegment::Q(x1, y1, x2, y2) => PathSegment::Q(x1 * scale_x, y1 * scale_y, x2 * scale_x, y2 * scale_y),
            PathSegment::C(x1, y1, x2, y2, x3, y3) => PathSegment::C(x1 * scale_x, y1 * scale_y, x2 * scale_x, y2 * scale_y, x3 * scale_x, y3 * scale_y),
            PathSegment::Z => PathSegment::Z,
        })
        .collect()
}

pub fn path_to_string(segments: PathSegments) -> String {
    use std::fmt::Write;
    let mut path_buf = String::from("");
    for segment in segments {
        match segment {
            PathSegment::M(x, y) => write!(path_buf, "M {} {} ", x, y).unwrap(),
            PathSegment::L(x, y) => write!(path_buf, "L {} {} ", x, y).unwrap(),
            PathSegment::Q(x1, y1, x2, y2) => write!(path_buf, "Q {} {} {} {} ", x1, y1, x2, y2).unwrap(),
            PathSegment::C(x1, y1, x2, y2, x3, y3) => write!(path_buf, "C {} {} {} {} {} {} ", x1, y1, x2, y2, x3, y3).unwrap(),
            PathSegment::Z => path_buf.push_str("Z "),
        }
    }
    path_buf
}

pub fn path_to_string_move(segments: PathSegments, move_x: f32, move_y: f32) -> String {
    use std::fmt::Write;
    let mut path_buf = String::from("");
    for segment in segments {
        match segment {
            PathSegment::M(x, y) => write!(path_buf, "M {} {} ", x + move_x, y + move_y).unwrap(),
            PathSegment::L(x, y) => write!(path_buf, "L {} {} ", x + move_x, y + move_y).unwrap(),
            PathSegment::Q(x1, y1, x2, y2) => write!(path_buf, "Q {} {} {} {} ", x1 + move_x, y1 + move_y, x2 + move_x, y2 + move_y).unwrap(),
            PathSegment::C(x1, y1, x2, y2, x3, y3) => write!(path_buf, "C {} {} {} {} {} {} ", x1 + move_x, y1 + move_y, x2 + move_x, y2 + move_y, x3 + move_x, y3 + move_y).unwrap(),
            PathSegment::Z => path_buf.push_str("Z "),
        }
    }
    path_buf
}

pub fn path_bounding_box(segments: PathSegments) -> (f32, f32, f32, f32) {
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;

    segments.iter().for_each(|segment| match segment {
        PathSegment::M(x, y) => {
            min_x = min_x.min(*x);
            min_y = min_y.min(*y);
            max_x = max_x.max(*x);
            max_y = max_y.max(*y);
        }
        PathSegment::L(x, y) => {
            min_x = min_x.min(*x);
            min_y = min_y.min(*y);
            max_x = max_x.max(*x);
            max_y = max_y.max(*y);
        }
        PathSegment::Q(x1, y1, x2, y2) => {
            min_x = min_x.min(*x1).min(*x2);
            min_y = min_y.min(*y1).min(*y2);
            max_x = max_x.max(*x1).max(*x2);
            max_y = max_y.max(*y1).max(*y2);
        }
        PathSegment::C(x1, y1, x2, y2, x3, y3) => {
            min_x = min_x.min(*x1).min(*x2).min(*x3);
            min_y = min_y.min(*y1).min(*y2).min(*y3);
            max_x = max_x.max(*x1).max(*x2).max(*x3);
            max_y = max_y.max(*y1).max(*y2).max(*y3);
        }
        PathSegment::Z => {}
    });

    (min_x, min_y, max_x, max_y)
}
