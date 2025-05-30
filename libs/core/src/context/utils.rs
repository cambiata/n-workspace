use crate::{sysitem::SysItemType, ItemId};

use super::CoreContext;

pub fn check_sysitems_parts_integrity(cx: &CoreContext, sysitems: Vec<ItemId>) -> Result<(), Box<dyn std::error::Error>> {
    let mut count: Option<u8> = None;

    for sysitemid in sysitems {
        let sysitems = cx.sysitems.borrow();
        let sysitem = sysitems.get(sysitemid as usize).unwrap();

        match sysitem.stype {
            SysItemType::Parts(ref parts, _) => {
                count = match count {
                    Some(c) => {
                        if c != parts.len() as u8 {
                            return Err("Part count error".into());
                        }
                        Some(parts.len() as u8)
                    }
                    None => Some(parts.len() as u8),
                };
            }
            SysItemType::Clefs(ref clefs) => {
                count = match count {
                    Some(c) => {
                        if c != clefs.len() as u8 {
                            return Err("Clefs count error".into());
                        }
                        Some(clefs.len() as u8)
                    }
                    None => Some(clefs.len() as u8),
                };
            }
            SysItemType::Barline => {}
            SysItemType::Other => {}
        };
    }

    Ok(())
}
