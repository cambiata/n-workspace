use graphics::rectangle::Rectangle;

#[derive(Debug)]
pub struct GridItem<T> {
    pub id: usize,
    pub colidx: usize,
    pub rowidx: usize,
    pub gitype: GridItemType<T>,
}

// pub type Rectangle = (f32, f32, f32, f32);

#[derive(Debug)]
pub enum GridItemType<T> {
    Empty,
    Rectangles(Vec<Rectangle>, Vec<T>),
}

#[derive(Debug)]
pub struct GridRow {
    pub rowidx: usize,
    pub item_ids: Vec<usize>,
}

#[derive(Debug)]
pub struct GridColumn {
    pub colidx: usize,
    pub item_ids: Vec<usize>,
}
