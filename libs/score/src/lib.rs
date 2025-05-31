use core::{context::CoreContext, sysitem::SysItemType};

pub fn build_score(cx: &CoreContext) -> Result<(), Box<dyn std::error::Error>> {
    // This function is a placeholder for the build process.
    // It can be used to perform any necessary setup or initialization.
    println!("Building the score library...");

    let sysitems = cx.sysitems.borrow();
    let _parts = cx.parts.borrow();
    let complexids = cx.map_partid_complexids.borrow();

    for (sysidx, sysitem) in sysitems.iter().enumerate() {
        println!("SysItem: {:?}", sysitem);
        match &sysitem.stype {
            SysItemType::Parts(part_ids, _sum_duration) => {
                for part_id in part_ids {
                    // let _part = &parts[*part_id as usize];
                    // match &part.ptype {
                    //     core::part::PartType::OneVoice(_voice) => {
                    //         println!("Simple part found in system {}", sysidx);
                    //     }
                    //     core::part::PartType::TwoVoice(_upper_voice, _lower_voice) => {
                    //         println!("Complex part found in system {}", sysidx);
                    //     }
                    //     core::part::PartType::OtherPart => {
                    //         todo!();
                    //     }
                    // }
                    for complex_id in complexids.get(part_id).expect("Part ID not found in map") {
                        let complex = &cx.complexes.borrow()[*complex_id as usize];
                        dbg!(&complex);
                    }
                }
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
