// pub type Position = (f32, f32);
pub type Rectangle = (f32, f32, f32, f32);

pub fn rectangle_overlap_x(left: Rectangle, right: Rectangle) -> f32 {
    if (right.1 >= left.1 + left.3) || (right.1 + right.3 <= left.1) {
        return 0.;
    }
    let left_right = left.0 + left.2 - right.0;
    left_right.max(0.)
}

pub fn rectangle_overlap_y(top: Rectangle, bottom: Rectangle) -> f32 {
    if (bottom.0 >= top.0 + top.2) || (bottom.0 + bottom.2 <= top.0) {
        return 0.;
    }
    let top_bottom = top.1 + top.3 - bottom.1;
    top_bottom.max(0.)
}

pub fn rectangles_overlap_x(lefts: &[Rectangle], rights: &[Rectangle]) -> f32 {
    let mut result: f32 = 0.;
    lefts.iter().for_each(|left| {
        rights.iter().for_each(|right| {
            let ol = rectangle_overlap_x(*left, *right);
            result = result.max(ol);
        });
    });
    result
}



pub fn widest_of_rectangles(rects: &[Rectangle]) -> f32 {
    let mut result: f32 = 0.;
    rects.iter().for_each(|rect| {
        let w = rect.2;
        result = result.max(w);
    });
    result
}
