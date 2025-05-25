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
        for rowidx in 0..self.rows.borrow().len() {
            self.calculate_minimal_col_spacing_for_row(rowidx);
        }
    }

    pub fn calculate_minimal_col_spacing_for_row(&self, rowidx: usize) {
        assert!(rowidx < self.rows.borrow().len(), "Row index out of bounds");
        let cx_items = self.items.borrow();

        let row = &self.rows.borrow()[rowidx];
        let items = &row.item_ids.iter().map(|id| &cx_items[*id]).collect::<Vec<_>>();

        let mut left_itemdata: Option<(usize, Vec<Rectangle>)> = None;
        let mut cols_overlaps = self.cols_overlaps.borrow_mut();
        //
        for (colidx, right_item) in items.into_iter().enumerate() {
            // println!("colidx:{} =======================================", colidx);

            match right_item.gitype {
                GridItemType::Empty => {
                    println!("colidx:{} Empty", colidx);
                }
                GridItemType::Rectangles(ref right_rects, ref _data) => {
                    //
                    match left_itemdata {
                        Some((_left_colidx, ref left_rects)) => {
                            let overlap_x = rectangles_overlap_x(left_rects, right_rects);

                            // dbg!(left_colidx, colidx);

                            let current_overlap_x = cols_overlaps[colidx];
                            if overlap_x > current_overlap_x {
                                cols_overlaps[colidx] = overlap_x;
                            }
                        }
                        None => {
                            //println!("Left item data is None");
                        }
                    }
                    // set prev_rects to the current item if it is a rectangle type
                    left_itemdata = Some((colidx, right_rects.clone()));
                }
            }
        }

        match items.last().unwrap().gitype {
            GridItemType::Empty => {
                println!("Last item is Empty");
            }
            GridItemType::Rectangles(ref last_rects, ref _data) => {
                // println!("Last item is Rectangles");
                let widest = widest_of_rectangles(last_rects);

                cols_overlaps[items.len()] = widest;
            }
        }
        //dbg!(&rowidx, &cols_overlaps);
    }
}
