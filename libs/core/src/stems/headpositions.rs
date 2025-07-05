use crate::{
    context::CoreContext,
    direction::DirectionUD,
    stems::stemitems::{StemHeadPosition, StemItem, StemType},
};

type HeadIdLevel = (usize, i8);

pub struct HeadPositionUtils;
impl HeadPositionUtils {
    pub fn calculate_head_positions(cx: &CoreContext) {
        HeadPositionUtils::set_head_positions(&cx.stemitems.borrow(), &mut cx.map_head_position.borrow_mut());
    }

    pub fn set_head_positions(stemitems: &[StemItem], map_head_position: &mut std::collections::BTreeMap<usize, StemHeadPosition>) {
        // let mut positions_map: Vec<(usize, StemHeadPosition)> = Vec::new();

        for stemitem in stemitems.iter() {
            if stemitem.direction.is_none() {
                println!("Problem: Stem item without direction: {:?}", stemitem);
                continue; // Skip items without a direction
            }

            match stemitem.stype {
                StemType::NoteWithoutStem(ref item) => {
                    let head_ids_levels = item.note.get_head_ids_and_levels().unwrap();
                    HeadPositionUtils::calc_head_positions(head_ids_levels, &stemitem.direction, map_head_position);

                    // positions_map.push(ids_positions);
                    // item.head_positions = Some(positions);
                }
                StemType::NoteWithStem(ref item) => {
                    let head_ids_levels = item.note.get_head_ids_and_levels().unwrap();
                    HeadPositionUtils::calc_head_positions(head_ids_levels, &stemitem.direction, map_head_position);

                    // positions_map.push((stemitem.id, positions));
                    // item.head_positions = Some(positions);
                }
                StemType::NotesBeamed(ref items) => {
                    // Handle beamed notes similarly
                    for item in items.iter() {
                        let head_ids_levels = item.note.get_head_ids_and_levels().unwrap();
                        HeadPositionUtils::calc_head_positions(head_ids_levels, &stemitem.direction, map_head_position);
                        // item.head_positions = Some(positions);
                        // positions_map.push((stemitem.id, positions));
                    }
                }
                _ => {}
            };
        }
    }

    #[allow(unused_variables)]
    fn calc_head_positions(head_ids_levels: Vec<HeadIdLevel>, direction: &Option<DirectionUD>, map_head_position: &mut std::collections::BTreeMap<usize, StemHeadPosition>) {
        if head_ids_levels.len() <= 1 {
            return;
        }

        match direction.as_ref().unwrap() {
            DirectionUD::Up => HeadPositionUtils::calc_head_positions_up(head_ids_levels, map_head_position),
            DirectionUD::Down => HeadPositionUtils::calc_head_positions_down(head_ids_levels, map_head_position),
        };
    }

    #[allow(unused_variables)]
    fn calc_head_positions_up(head_ids_levels: Vec<HeadIdLevel>, map_head_position: &mut std::collections::BTreeMap<usize, StemHeadPosition>) {
        // println!("calc_head_positions_up: {:?}", head_ids_levels);
        let mut ids_levels = head_ids_levels.clone();
        ids_levels.reverse();

        let mut positions: Vec<(usize, StemHeadPosition)> = vec![(ids_levels[0].0, StemHeadPosition::Center)];

        for head_id_level in ids_levels.windows(2) {
            let diff = head_id_level[0].1 - head_id_level[1].1;
            let id = head_id_level[1].0;

            // dbg!(&head_id_level, &diff);
            match &positions.last().unwrap().1 {
                StemHeadPosition::Center => {
                    if diff <= 1 {
                        positions.push((id, StemHeadPosition::Right));
                    } else {
                        positions.push((id, StemHeadPosition::Center));
                    }
                }
                StemHeadPosition::Right => {
                    positions.push((id, StemHeadPosition::Center));
                }
                _ => {
                    println!("Unexpected position: {:?}", positions.last().unwrap());
                }
            }
        }

        for position in positions.iter() {
            if position.1 != StemHeadPosition::Center {
                map_head_position.insert(position.0, position.1.clone());
            }
        }
    }

    #[allow(unused_variables)]
    fn calc_head_positions_down(head_ids_levels: Vec<HeadIdLevel>, map_head_position: &mut std::collections::BTreeMap<usize, StemHeadPosition>) {
        // println!("calc_head_positions_down: {:?}", head_ids_levels);

        let mut ids_levels = head_ids_levels.clone();
        ids_levels.reverse();

        let mut positions: Vec<(usize, StemHeadPosition)> = vec![(ids_levels[0].0, StemHeadPosition::Center)];

        for head_id_level in ids_levels.windows(2) {
            let diff = head_id_level[0].1 - head_id_level[1].1;
            let id = head_id_level[1].0;

            // dbg!(&head_id_level, &diff);
            match &positions.last().unwrap().1 {
                StemHeadPosition::Center => {
                    if diff <= 1 {
                        positions.push((id, StemHeadPosition::Left));
                    } else {
                        positions.push((id, StemHeadPosition::Center));
                    }
                }
                StemHeadPosition::Left => {
                    positions.push((id, StemHeadPosition::Center));
                }
                _ => {
                    println!("Unexpected position: {:?}", positions.last().unwrap());
                }
            }
        }

        for position in positions.iter() {
            if position.1 != StemHeadPosition::Center {
                map_head_position.insert(position.0, position.1.clone());
            }
        }
    }
}
