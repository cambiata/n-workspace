pub mod complex;
pub mod constants;
pub mod glyphitem;

use core::{context::CoreContext, duration::SumDuration, part::complex::ComplexInfo, sysitem::SysItemType, ItemId};
use std::collections::BTreeMap;

use complex::create_rectangles_complex;

pub fn build_sysitems(cx: &CoreContext) -> Result<(), Box<dyn std::error::Error>> {
    let sysitems = cx.sysitems.borrow();
    let _parts = cx.parts.borrow();

    for (sysidx, sysitem) in sysitems.iter().enumerate() {
        // println!("SysItem: {column_index} {:?}", sysitem);
        match &sysitem.stype {
            SysItemType::Parts(_part_ids, _sum_duration, _complexes_infos, _positions_durations) => {
                let _x = build_sysitem_parts(cx, _part_ids, _sum_duration, _complexes_infos, _positions_durations);
            }

            SysItemType::Clefs(_clefs) => {
                println!("Clef found in sysitem {}", sysidx);
            }
            SysItemType::Barline => {
                println!("Barline found in sysitem {}", sysidx);
            }
            SysItemType::Other => {
                println!("Other item found in sysitem {}", sysidx);
            }
        }
    }
    Ok(())
}

pub fn build_sysitem_parts(
    cx: &CoreContext,
    _parts_ids: &Vec<ItemId>,
    _sum_duration: &SumDuration,
    complexes_infos: &Vec<BTreeMap<usize, ComplexInfo>>,
    positions_durations: &BTreeMap<usize, usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let complexes = cx.complexes.borrow();

    for (pos, dur) in positions_durations.iter() {
        println!("- - Position: {}, Duration: {}", pos, dur);
        for (partidx, _part_complexes) in complexes_infos.iter().enumerate() {
            if let Some(complex_info) = _part_complexes.get(pos) {
                println!("- - - Part {}: Complex Info: {:?}", partidx, complex_info);
                let complex_id = complex_info.0;
                let complex = &complexes[complex_id];

                let complex_rectangles = create_rectangles_complex(cx, partidx, complex);
                dbg!(complex_rectangles);
            } else {
                println!("- - - Part {}: No complex info at position {}", partidx, pos);
            }
        }
    }

    Ok(())
}
