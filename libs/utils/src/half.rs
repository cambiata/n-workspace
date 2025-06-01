pub trait F32Half {
    fn half(self) -> f32;
}

impl F32Half for f32 {
    fn half(self) -> f32 {
        self / 2.0
    }
}
