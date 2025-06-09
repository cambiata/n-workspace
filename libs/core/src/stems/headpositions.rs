use crate::{
    complex::ComplexType,
    context::CoreContext,
    direction::{self, DirectionUD},
    stems::stemitems::{StemHeadPosition, StemType},
};

pub fn calculate_head_positions(cx: &CoreContext) {
    let mut positions_map: Vec<(usize, Vec<StemHeadPosition>)> = Vec::new();

    for stemitem in cx.stemitems.borrow().iter() {
        if stemitem.direction.is_none() {
            println!("Problem: Stem item without direction: {:?}", stemitem);
            continue; // Skip items without a direction
        }

        match stemitem.stype {
            StemType::NoteWithoutStem(ref item) => {
                let head_levels = item.note.get_head_levels().unwrap();
                let positions = calc_head_positions(head_levels, &stemitem.direction);
                positions_map.push((stemitem.id, positions));
                // item.head_positions = Some(positions);
            }
            StemType::NoteWithStem(ref item) => {
                let head_levels = item.note.get_head_levels().unwrap();
                let positions = calc_head_positions(head_levels, &stemitem.direction);
                positions_map.push((stemitem.id, positions));
                // item.head_positions = Some(positions);
            }
            StemType::NotesBeamed(ref items) => {
                // Handle beamed notes similarly
                for item in items.iter() {
                    let head_levels = item.note.get_head_levels().unwrap();
                    let positions = calc_head_positions(head_levels, &stemitem.direction);
                    // item.head_positions = Some(positions);
                    positions_map.push((stemitem.id, positions));
                }
            }
            _ => {}
        };
    }

    for (id, positions) in positions_map {
        let mut stemitems = cx.stemitems.borrow_mut();
        if let Some(stemitem) = stemitems.iter_mut().find(|s| s.id == id) {
            match &mut stemitem.stype {
                StemType::NoteWithoutStem(item) => item.head_positions = Some(positions),
                StemType::NoteWithStem(item) => item.head_positions = Some(positions),
                StemType::NotesBeamed(items) => {
                    for item in items.iter_mut() {
                        item.head_positions = Some(positions.clone());
                    }
                }
                _ => {}
            }
        }
    }
}

fn calc_head_positions(head_levels: Vec<i8>, direction: &Option<DirectionUD>) -> Vec<StemHeadPosition> {
    if head_levels.len() == 1 {
        return vec![StemHeadPosition::Center];
    }
    match direction.as_ref().unwrap() {
        DirectionUD::Up => calc_head_positions_up(head_levels),
        DirectionUD::Down => calc_head_positions_down(head_levels),
    }
}

fn calc_head_positions_up(head_levels: Vec<i8>) -> Vec<StemHeadPosition> {
    let mut positions: Vec<StemHeadPosition> = vec![StemHeadPosition::Center];
    let mut levels = head_levels.clone();
    levels.reverse();
    println!("UP: {:?}", levels);

    for heads in levels.windows(2) {
        let diff = heads[0] - heads[1];
        dbg!(&heads, &diff);
        match &positions.last().unwrap() {
            StemHeadPosition::Center => {
                if diff <= 1 {
                    positions.push(StemHeadPosition::Right);
                } else {
                    positions.push(StemHeadPosition::Center);
                }
            }
            StemHeadPosition::Right => {
                positions.push(StemHeadPosition::Center);
            }
            _ => {
                println!("Unexpected position: {:?}", positions.last().unwrap());
            }
        }
    }
    dbg!(&positions);

    positions
}

fn calc_head_positions_down(head_levels: Vec<i8>) -> Vec<StemHeadPosition> {
    let mut levels = head_levels.clone();
    levels.reverse();
    println!("DOWN: {:?}", levels);

    let mut positions: Vec<StemHeadPosition> = vec![StemHeadPosition::Center];

    for heads in levels.windows(2) {
        let diff = heads[0] - heads[1];
        match &positions.last().unwrap() {
            StemHeadPosition::Center => {
                if diff <= 1 {
                    positions.push(StemHeadPosition::Left);
                } else {
                    positions.push(StemHeadPosition::Center);
                }
            }
            StemHeadPosition::Left => {
                positions.push(StemHeadPosition::Center);
            }
            _ => {
                println!("Unexpected position: {:?}", positions.last().unwrap());
            }
        }
    }
    dbg!(&positions);

    positions
}

#[cfg(test)]
mod tests2 {

    use super::calc_head_positions_up;

    #[test]
    fn example() {
        let levels: Vec<i8> = vec![0, 1, 3];
        let positions = calc_head_positions_up(levels);
    }
}
