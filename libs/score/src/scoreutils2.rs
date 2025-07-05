use core::{
    complex,
    context::CoreContext,
    hpart::{HPartMusicType, HPartType},
};
use std::collections::BTreeMap;

use crate::scorecontext::ScoreContext;

pub struct ScoreUtils2;
impl ScoreUtils2 {
    pub fn build(scx: &ScoreContext, cx: &CoreContext) {
        let cx_hparts = cx.hparts.borrow();
        let cx_complexes = cx.complexes.borrow();

        let map_complex: BTreeMap<usize, Vec<Option<usize>>> = BTreeMap::new();

        for (column_idx, item) in cx.columns.borrow().iter().enumerate() {
            dbg!(item);
            let column_hparts_ids = item.hpart_ids.clone();

            let first_hpart = &cx_hparts[column_hparts_ids[0]];
            match &first_hpart.parttype {
                HPartType::Music(_, complexes, _) => {
                    //
                    for hpart_id in column_hparts_ids.iter() {
                        let hpart = &cx_hparts[*hpart_id];
                        match &hpart.parttype {
                            HPartType::Music(_, complexes, _) => {
                                // dbg!(complexes);
                                for complex_id in complexes.iter() {
                                    let complex = &cx_complexes[*complex_id];
                                    dbg!(complex);
                                    // Here you can process the complex as needed
                                    // For example, you might want to store it in a map or perform some calculations
                                    // map_complex.entry(column_idx).or_default().push(Some(*complex_id));
                                }
                            }
                            _ => {
                                println!("Unsupported part type in column: {:?}", hpart.parttype);
                                continue;
                            }
                        }
                    }
                }

                _ => {}
            };
        }
    }
}
