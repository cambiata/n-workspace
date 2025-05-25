use graphics::path::PathSegment;
use rusttype::OutlineBuilder;

pub struct PathBuilder {
    pub x: f32,
    pub y: f32,
    pub segments: Vec<PathSegment>,
}

impl PathBuilder {
    pub fn get_segments(&self) -> Vec<PathSegment> {
        self.segments.clone()
    }
}

impl OutlineBuilder for PathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.segments.push(PathSegment::M(self.x + x, self.y + y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.segments.push(PathSegment::L(self.x + x, self.y + y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.segments.push(PathSegment::Q(self.x + x1, self.y + y1, self.x + x, self.y + y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.segments.push(PathSegment::C(self.x + x1, self.y + y1, self.x + x2, self.y + y2, self.x + x, self.y + y));
    }

    fn close(&mut self) {
        self.segments.push(PathSegment::Z);
    }
}
