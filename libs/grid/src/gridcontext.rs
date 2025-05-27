use std::cell::RefCell;
use std::fmt::Debug;

use crate::griditem::{GridColumn, GridItem, GridItemType, GridRow};
use graphics::rectangle::{rectangles_overlap_x, widest_of_rectangles, Rectangle};

#[derive(Debug)]
pub struct GridContext<T>
where
    T: Debug + Copy + Clone,
{
    pub items: RefCell<Vec<GridItem<T>>>,
    pub cols: RefCell<Vec<GridColumn>>,
    pub cols_overlaps: RefCell<Vec<f32>>,
    pub rows: RefCell<Vec<GridRow>>,
    // pub xxx: RefCell<BTreeMap<ItemId, usize>>,
}

impl<T> GridContext<T>
where
    T: Debug + Copy + Clone,
{
    pub fn new() -> &'static GridContext<T> {
        let cx = GridContext {
            items: RefCell::new(Vec::new()),
            cols: RefCell::new(Vec::new()),
            cols_overlaps: RefCell::new(Vec::new()),
            rows: RefCell::new(Vec::new()),
            // xxx: RefCell::new(BTreeMap::new()),
        };
        Box::leak(Box::new(cx))
    }

    pub fn add_items(&self, items: Vec<Vec<GridItemType<T>>>) {
        let mut cx_items = self.items.borrow_mut();
        let mut cx_cols = self.cols.borrow_mut();
        let mut cx_rows = self.rows.borrow_mut();

        let mut colrowids: Vec<Vec<usize>> = Vec::new();
        let rowcount = items.len();
        let colcount = items[0].len();
        for rowidx in 0..rowcount {
            let ccount = items[rowidx].len();
            if ccount != colcount {
                panic!("Row {} has different column count than the first row", rowidx);
            }
        }

        for (rowidx, rowitems) in items.into_iter().enumerate() {
            let mut rowids = Vec::<usize>::new();
            for (colidx, colitem) in rowitems.into_iter().enumerate() {
                let itemid = cx_items.len();
                let item = GridItem {
                    id: itemid,
                    colidx,
                    rowidx,
                    gitype: colitem,
                };
                cx_items.push(item);
                rowids.push(itemid);
            }
            let gridrow = GridRow { rowidx, item_ids: rowids.to_vec() };
            cx_rows.push(gridrow);
            colrowids.push(rowids.to_vec());
        }

        for colidx in 0..colcount {
            let mut colids = Vec::<usize>::new();
            for rowidx in 0..rowcount {
                colids.push(colrowids[rowidx][colidx]);
            }

            let gridcol = GridColumn { colidx, item_ids: colids.to_vec() };
            cx_cols.push(gridcol);
        }

        for _ in 0..colcount {
            self.cols_overlaps.borrow_mut().push(0.0);
        }
        self.cols_overlaps.borrow_mut().push(0.0); // one extra for the last column

        dbg!(&rowcount, &colcount);
    }

    pub fn calculate_minimal_col_spacing(&self) {
        let cols: Vec<usize> = (0..self.cols.borrow().len()).collect();
        let self_items = self.items.borrow();
        let self_rows = self.rows.borrow();
        let mut self_cols_overlaps = self.cols_overlaps.borrow_mut();

        // let mut current_left_col_x = 0.0;
        // let mut current_right_col_x = 0.0;
        for colidx in cols.windows(2) {
            let left_colidx = colidx[0];
            let right_colidx = colidx[1];

            // current_left_col_x += cols_overlaps[left_colidx];
            // current_right_col_x += cols_overlaps[right_colidx];

            for (rowidx, row) in self_rows.iter().enumerate() {
                println!("Rowidx {rowidx} --------------------------------------");
                let left_item = &self_items[row.item_ids[left_colidx]];
                let right_item = &self_items[row.item_ids[right_colidx]];
                match (&left_item.gitype, &right_item.gitype) {
                    (GridItemType::Rectangles(ref left_rects, _), GridItemType::Rectangles(ref right_rects, _)) => {
                        let overlap_x = rectangles_overlap_x(left_rects, right_rects);
                        dbg!(overlap_x);

                        if overlap_x > self_cols_overlaps[right_colidx] {
                            self_cols_overlaps[right_colidx] = overlap_x;
                        }
                    }
                    (GridItemType::Empty, _) => {
                        println!("Left item is Empty");
                    }
                    (_, GridItemType::Empty) => {
                        println!("Right item is Empty");
                    }
                }
            }
        }
        dbg!(self_cols_overlaps);
    }
}
