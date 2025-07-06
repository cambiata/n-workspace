use core::hpart::{HPartItem, HPartType};
use std::collections::{BTreeMap, BTreeSet};

pub struct BuildUtils;
impl BuildUtils {
    pub fn get_complexes_information(
        cx: &core::context::CoreContext,
        hparts: &[&HPartItem],
    ) -> Result<(BTreeSet<usize>, BTreeMap<usize, Vec<usize>>, BTreeMap<(usize, usize), usize>), Box<dyn std::error::Error>> {
        let cx_complexes = cx.complexes.borrow();

        // Collect positions and map them to hpart indices
        let mut positions: BTreeSet<usize> = BTreeSet::new();
        let mut map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        let mut map_ids: BTreeMap<(usize, usize), usize> = BTreeMap::new();

        hparts.iter().enumerate().for_each(|(part_idx, hpart)| {
            if let HPartType::Music { complexes, .. } = &hpart.hptype {
                map.insert(part_idx, Vec::new());
                for complex_id in complexes {
                    let complex = cx_complexes.get(*complex_id).unwrap();
                    positions.insert(complex.position);
                    map.get_mut(&part_idx).unwrap().push(complex.position);
                    map_ids.insert((part_idx, complex.position), *complex_id);
                }
            } else {
                panic!("Expected HPartType::Music, found {:?}", hpart.hptype);
            }
        });

        Ok((positions, map, map_ids))
    }
}
