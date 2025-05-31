use core::{context::CoreContext, sysitem::SysItemType};

pub fn build_score(cx: &CoreContext) -> Result<(), Box<dyn std::error::Error>> {
    // This function is a placeholder for the build process.
    // It can be used to perform any necessary setup or initialization.
    println!("Building the score library...");

    let sysitems = cx.sysitems.borrow();
    let _parts = cx.parts.borrow();
    // let complexids = cx.map_partid_complexids.borrow();

    for (sysidx, sysitem) in sysitems.iter().enumerate() {
        println!("SysItem: {:?}", sysitem);
        match &sysitem.stype {
            SysItemType::Parts(_part_ids, _sum_duration, _complexes_infos) => {
                // for part_id in part_ids {
                //     let _part = &parts[*part_id as usize];
                //     dbg!(&_part);
                // }
            }
            SysItemType::Clefs(_clefs) => {
                println!("Clef found in system {}", sysidx);
            }
            SysItemType::Barline => {
                println!("Barline found in system {}", sysidx);
            }
            SysItemType::Other => {
                println!("Other item found in system {}", sysidx);
            }
        }
    }
    Ok(())
}
