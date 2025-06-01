pub trait F32ExtHalf {
    fn half(self) -> f32;
}

impl F32ExtHalf for f32 {
    fn half(self) -> f32 {
        self / 2.0
    }
}
