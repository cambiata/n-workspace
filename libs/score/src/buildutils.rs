use core::hpart::{HPartItem, HPartType};
use std::collections::{BTreeMap, BTreeSet};

pub struct BuildUtils;
impl BuildUtils {
    pub fn get_complexes_information(
        cx: &core::context::CoreContext,
        hparts: &[&HPartItem],
        duration: usize,
    ) -> Result<(Vec<usize>, Vec<usize>, BTreeMap<(usize, usize), usize>), Box<dyn std::error::Error>> {
        let cx_complexes = cx.complexes.borrow();

        // Collect positions and map them to hpart indices
        let mut positions: BTreeSet<usize> = BTreeSet::new();
        // let mut positions: Vec<usize> = Vec::new();
        // let mut map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        let mut map_ids: BTreeMap<(usize, usize), usize> = BTreeMap::new();

        hparts.iter().enumerate().for_each(|(part_idx, hpart)| {
            if let HPartType::Music { complexes, .. } = &hpart.hptype {
                // map.insert(part_idx, Vec::new());
                for complex_id in complexes {
                    let complex = cx_complexes.get(*complex_id).unwrap();
                    positions.insert(complex.position);
                    // map.get_mut(&part_idx).unwrap().push(complex.position);
                    map_ids.insert((part_idx, complex.position), *complex_id);
                }
            } else {
                panic!("Expected HPartType::Music, found {:?}", hpart.hptype);
            }
        });

        let positions = positions.into_iter().collect::<Vec<usize>>();

        let mut positions_incl_duration: Vec<usize> = positions.clone();
        positions_incl_duration.push(duration);
        dbg!(&positions_incl_duration);

        let mut durations: Vec<usize> = Vec::new();
        for left_right in positions_incl_duration.windows(2) {
            let left = left_right[0];
            let right = left_right[1];
            let duration = right - left;
            durations.push(duration);
        }
        dbg!(&durations);
        dbg!(&positions);

        // calculat allotments...

        Ok((positions, durations, map_ids))
    }
}
