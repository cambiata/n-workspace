pub trait SliceExt<T> {
    // fn slice(&self, start: usize, end: usize) -> &[T];
    fn second(&self) -> Option<&T>;
    fn third(&self) -> Option<&T>;
    fn fourth(&self) -> Option<&T>;
}

impl<T> SliceExt<T> for [T] {
    fn second(&self) -> Option<&T> {
        self.get(1)
    }
    fn third(&self) -> Option<&T> {
        self.get(2)
    }
    fn fourth(&self) -> Option<&T> {
        self.get(3)
    }
}
