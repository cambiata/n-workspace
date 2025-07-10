use std::cell::RefCell;
use std::fmt::Debug;

use crate::griditem::{GridColumn, GridItem, GridItemType, GridRow};
use graphics::rectangle::{rectangles_overlap_x, Rectangle};
use utils::f32_ext::round::F32ExtRound2;

#[derive(Debug)]
pub struct GridContext<T>
where
    T: Debug + Clone,
{
    pub items: RefCell<Vec<GridItem<T>>>,
    pub cols: RefCell<Vec<GridColumn>>,
    pub cols_widths: RefCell<Vec<f32>>,
    pub rows: RefCell<Vec<GridRow>>,
}

#[allow(unused_variables)]
impl<T> GridContext<T>
where
    T: Debug + Clone,
{
    pub fn new() -> &'static GridContext<T> {
        let cx = GridContext {
            items: RefCell::new(Vec::new()),
            cols: RefCell::new(Vec::new()),
            cols_widths: RefCell::new(Vec::new()),

            rows: RefCell::new(Vec::new()),
        };
        Box::leak(Box::new(cx))
    }

    pub fn add_items(&self, items: Vec<Vec<GridItemType<T>>>) -> Result<(usize, usize), Box<dyn std::error::Error>> {
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

        // for _ in 0..colcount {
        //     self.cols_overlaps.borrow_mut().push(0.0);
        // }
        // self.cols_overlaps.borrow_mut().push(0.0); // one extra for the last column?

        Ok((colcount, rowcount))
    }

    pub fn set_cols_widths(&self, spacing: Vec<f32>) -> Result<(), Box<dyn std::error::Error>> {
        *self.cols_widths.borrow_mut() = spacing;
        Ok(())
    }

    //----------------------------------------------------------

    pub fn handle_column_spacing(&self, allotments: &Vec<f32>) -> Result<(), Box<dyn std::error::Error>> {
        let minimal_spacing = self.calculate_minimal_col_spacing()?;
        let minimal_width = minimal_spacing.iter().sum::<f32>();
        dbg!(&minimal_width);
        dbg!(&minimal_spacing);

        let alloted_spacing = self.calculate_alloted_col_spacing(allotments, 1.9)?;
        // dbg!(&alloted_spacing);

        let final_spacing = self.calculate_final_col_spacing(&minimal_spacing, &alloted_spacing)?;
        let final_width = final_spacing.iter().sum::<f32>();
        dbg!(&final_width);
        dbg!(&final_spacing);

        self.set_cols_widths(final_spacing)?;

        Ok(())
    }

    fn calculate_alloted_col_spacing(&self, allotments: &Vec<f32>, factor: f32) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        let alloted_overlaps = calculate_overlaps_with_factor(&allotments, factor);
        let alloted_width = alloted_overlaps.iter().sum::<f32>();
        Ok(alloted_overlaps)
    }

    pub fn calculate_minimal_col_spacing(&self) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        type PrevRectData = Option<(usize, Vec<Rectangle>)>;

        let colindexes: Vec<usize> = (0..self.cols.borrow().len()).collect();
        let self_items = self.items.borrow();
        let self_rows = self.rows.borrow();
        let colcount = self.cols.borrow().len();

        // let mut cols_overlaps = self.cols_overlaps.borrow_mut();
        let mut spacing: Vec<f32> = Vec::new();
        //-----------------------------------------------
        // Create cols_overlaps - should be deleted later?
        for _ in 0..colcount {
            spacing.push(0.0);
        }
        spacing.push(0.0);

        //-----------------------------------------------
        let mut prev_rect_data: Vec<PrevRectData> = vec![None; self.rows.borrow().len()];
        // pass one: calculate overlaps for each column pair
        for colidx in colindexes.windows(2) {
            let left_colidx = colidx[0];
            let right_colidx = colidx[1];
            for (rowidx, row) in self_rows.iter().enumerate() {
                // println!("colidx {left_colidx}-{right_colidx}, Rowidx {rowidx} --------");
                let left_item = &self_items[row.item_ids[left_colidx]];
                let right_item = &self_items[row.item_ids[right_colidx]];
                match (&left_item.gitype, &right_item.gitype) {
                    (GridItemType::Rectangles(ref left_items), GridItemType::Rectangles(ref right_items)) => {
                        // println!("- Both items are Rectangles");
                        let left_rects = &left_items.iter().map(|(r, _)| *r).collect::<Vec<Rectangle>>();
                        let right_rects = &right_items.iter().map(|(r, _)| *r).collect::<Vec<Rectangle>>();
                        let overlap_x = rectangles_overlap_x(left_rects, right_rects);
                        // dbg!(overlap_x);

                        if overlap_x > spacing[right_colidx] {
                            spacing[right_colidx] = overlap_x;
                        }
                        // Store the right rectangles for later use
                        prev_rect_data[rowidx] = Some((right_colidx, right_rects.clone()));
                    }
                    (GridItemType::Empty, GridItemType::Rectangles(ref right_items)) => {
                        let right_rects = &right_items.iter().map(|(r, _)| *r).collect::<Vec<Rectangle>>();
                        // println!("- (Empty, Rectangles) - Right item is Rectangles");
                        if let Some((prev_colidx, prev_rects)) = &prev_rect_data[rowidx] {
                            // println!("- Compare right_rects column {right_colidx} with previous rectangles at column {prev_colidx}");

                            let sum_cols_overlaps = ((prev_colidx + 1)..=right_colidx).map(|i| spacing[i]).sum::<f32>();
                            let overlap_x = rectangles_overlap_x(prev_rects, right_rects);
                            let factor = (overlap_x - sum_cols_overlaps).max(0.0);
                            if factor > 0.0 {
                                spacing[right_colidx] = spacing[right_colidx] + factor;
                            }
                        } else {
                            // println!("- No previous rectangles to compare with");
                        }
                    }
                    (GridItemType::Rectangles(ref left_items), GridItemType::Empty) => {
                        let left_rects = &left_items.iter().map(|(r, _)| *r).collect::<Vec<Rectangle>>();
                        // println!("- (Rectangles, Empty) - Left item is Rectangles");
                        // println!("- Store left_rects for later use");
                        prev_rect_data[rowidx] = Some((left_colidx, left_rects.clone()));
                    }
                    (GridItemType::Empty, GridItemType::Empty) => {
                        // println!("- Both items are empty, no overlap to calculate");
                    }
                }
                // dbg!(&prev_rect_data);
            }
        }

        // calculate last column's width
        let last_colidx = self.cols.borrow().len() - 1;
        let mut max_w: f32 = 0.0;
        for rowidx in 0..self_rows.len() {
            // println!("Row {rowidx} - Last column {last_colidx} --------");
            let item_id = &self_rows[rowidx].item_ids[last_colidx];
            let item = &self_items[*item_id];

            match &item.gitype {
                GridItemType::Rectangles(ref items) => {
                    let rects = &items.iter().map(|(r, _)| *r).collect::<Vec<Rectangle>>();
                    // dbg!(rects);
                    for rect in rects.iter() {
                        max_w = max_w.max(rect.2 + rect.0);
                    }

                    // Ensure the last column has enough space for the widest rectangle
                    if spacing[last_colidx + 1] < max_w {
                        spacing[last_colidx + 1] = max_w;
                    }
                }
                GridItemType::Empty => {
                    // No action needed for empty items
                }
            }
        }

        Ok(spacing)
    }

    fn calculate_final_col_spacing(&self, minimal_spacing: &[f32], alloted_spacing: &[f32]) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        let final_spacing = minimal_spacing.iter().zip(alloted_spacing.iter()).map(|(minimal, alloted)| minimal.max(*alloted)).collect::<Vec<f32>>();
        Ok(final_spacing)
    }
}

fn calculate_overlaps_with_factor(allotments: &[f32], factor: f32) -> Vec<f32> {
    let mut overlaps = vec![0.0; allotments.len() + 1];
    for (idx, allotment) in allotments.iter().enumerate() {
        let calculated_width = allotment * factor;
        if calculated_width > overlaps[idx + 1] {
            overlaps[idx + 1] = calculated_width.r2();
        }
    }
    overlaps
}
