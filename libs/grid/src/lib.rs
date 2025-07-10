pub mod gridcontext;
pub mod griditem;

#[derive(Debug, Clone, Copy)]
pub enum TestEnum {
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    Purple,
}

#[cfg(test)]
mod tests2 {
    use crate::{gridcontext::GridContext, griditem::GridItemType, TestEnum};

    #[test]
    fn example2() {
        let cx: &'static GridContext<TestEnum> = GridContext::<TestEnum>::new();

        // let items: Vec<Vec<GridItemType<TestEnum>>> = vec![
        //     vec![
        //         GridItemType::Rectangles(vec![(0.0, 0.0, 5.0, 10.0)], vec![TestEnum::Blue]),
        //         GridItemType::Rectangles(vec![(0.0, 0.0, 10.0, 10.0)], vec![TestEnum::Red]),
        //     ],
        //     vec![GridItemType::Rectangles(vec![(5.0, 5.0, 10.0, 10.0)], vec![TestEnum::Blue]), GridItemType::Empty],
        //     vec![GridItemType::Rectangles(vec![(10.0, 10.0, 10.0, 10.0)], vec![TestEnum::Green]), GridItemType::Empty],
        // ];

        let items = vec![vec![
            GridItemType::Rectangles(vec![((0.0, 0.0, 5.0, 10.0), TestEnum::Blue)]),
            GridItemType::Rectangles(vec![((0.0, 0.0, 10.0, 10.0), TestEnum::Red)]),
            GridItemType::Rectangles(vec![((0.0, 0.0, 5.0, 10.0), TestEnum::Orange)]),
        ]];

        cx.add_items(items);
        // cx.calculate_minimal_col_spacing_for_row(0);
        cx.calculate_minimal_col_spacing();
        dbg!(cx.cols_widths.borrow());
    }
}
