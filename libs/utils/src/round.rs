pub trait F32Round2 {
    fn r2(self) -> f32;
}

impl F32Round2 for f32 {
    fn r2(self) -> f32 {
        (self * 100.0).round() / 100.0
    }
}
