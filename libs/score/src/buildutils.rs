use core::{
    duration::NoteDuration,
    hpart::{HPartItem, HPartType},
};
use std::collections::{BTreeMap, BTreeSet};

use utils::f32_ext::round::F32ExtRound2;

pub struct BuildUtils;
impl BuildUtils {
    pub fn get_complexes_positions_allotments(
        cx: &core::context::CoreContext,
        hparts: &[&HPartItem],
        duration: usize,
    ) -> Result<(Vec<usize>, Vec<usize>, Vec<f32>, BTreeMap<(usize, usize), usize>), Box<dyn std::error::Error>> {
        let cx_complexes = cx.complexes.borrow();

        // Collect positions and map them to hpart indices
        let mut positions: BTreeSet<usize> = BTreeSet::new();
        // let mut positions: Vec<usize> = Vec::new();
        // let mut map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        let mut map_ids: BTreeMap<(usize, usize), usize> = BTreeMap::new();

        hparts.iter().enumerate().for_each(|(part_idx, hpart)| {
            if let HPartType::Music { complexes, mtype: _, .. } = &hpart.hptype {
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

        let mut durations: Vec<usize> = Vec::new();
        for left_right in positions_incl_duration.windows(2) {
            let left = left_right[0];
            let right = left_right[1];
            let duration = right - left;
            durations.push(duration);
        }
        // dbg!(&durations);
        // dbg!(&positions);

        //------------------------------
        // calculat allotments...

        let allotments: Vec<f32> = durations.iter().map(|d| Spacing::relative(*d)).collect();
        Ok((positions, durations, allotments, map_ids))
    }
}

struct Spacing;
impl Spacing {
    #[allow(dead_code)]
    pub fn linear(dur: usize) -> f32 {
        (dur as f32).r2() // Scale factor for spacing
    }

    pub fn relative(dur: usize) -> f32 {
        let factor = 4.0;
        let dur: NoteDuration = NoteDuration::try_from(dur).unwrap();
        let space = match dur {
            NoteDuration::D1 => 7.0,
            NoteDuration::D2Dot => 6.0,
            NoteDuration::D2 => 5.0,
            NoteDuration::D4Dot => 4.0,
            NoteDuration::D2Tri => 3.75,
            NoteDuration::D4 => 3.5,
            NoteDuration::D8Dot => 3.0,
            NoteDuration::D4Tri => 2.75,
            NoteDuration::D8 => 2.5,
            NoteDuration::D16Dot => 2.35,
            NoteDuration::D8Tri => 2.15,
            NoteDuration::D16 => 2.0,
            NoteDuration::D16Tri => 1.75,
            NoteDuration::D32 => 1.5,
            _ => {
                println!("Implement relative spacing for all note durations {:?}", dur);
                3.5
            }
        };
        (space * factor).r2() // Scale factor for spacing
    }
}
