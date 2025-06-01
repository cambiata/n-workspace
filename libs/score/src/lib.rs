use core::{context::CoreContext, duration::SumDuration, part::complex::ComplexInfo, sysitem::SysItemType, ItemId};
use std::collections::BTreeMap;

pub fn build_sysitems(cx: &CoreContext) -> Result<(), Box<dyn std::error::Error>> {
    let sysitems = cx.sysitems.borrow();
    let _parts = cx.parts.borrow();

    for (sysidx, sysitem) in sysitems.iter().enumerate() {
        println!("SysItem: {:?}", sysitem);
        match &sysitem.stype {
            SysItemType::Parts(_part_ids, _sum_duration, _complexes_infos, _positions_durations) => {
                // dbg!(&_positions_durations);
                let _x = build_sysitem_parts(cx, _part_ids, _sum_duration, _complexes_infos, _positions_durations);
                // for part_id in part_ids {
                //     let _part = &parts[*part_id as usize];
                //     dbg!(&_part);
                // }
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
    _cx: &CoreContext,
    _parts_ids: &Vec<ItemId>,
    _sum_duration: &SumDuration,
    complexes_infos: &Vec<BTreeMap<usize, ComplexInfo>>,
    positions_durations: &BTreeMap<usize, usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    for (pos, dur) in positions_durations.iter() {
        println!("- - Position: {}, Duration: {}", pos, dur);
        for (partidx, _part_complexes) in complexes_infos.iter().enumerate() {
            if let Some(complex_info) = _part_complexes.get(pos) {
                println!("- - - Part {}: Complex Info: {:?}", partidx, complex_info);
            } else {
                println!("- - - Part {}: No complex info at position {}", partidx, pos);
            }
        }
    }

    Ok(())
}
