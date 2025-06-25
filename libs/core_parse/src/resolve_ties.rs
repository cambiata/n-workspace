use core::{
    context::CoreContext,
    sysitem::{SysitemId, SysitemPosition, VecPartNotes},
    ties::{ResolvedTieFrom, ResolvedTieTo, TieFrom},
};
use std::error::Error;

pub fn handle_ties(cx: &CoreContext, partnotes_data: &Vec<(VecPartNotes, VecPartNotes)>) -> Result<(), Box<dyn Error>> {
    for (partnotes_upper, partnotes_lower) in partnotes_data {
        handle_partnotes(cx, 0, &partnotes_upper)?;
        handle_partnotes(cx, 1, &partnotes_lower)?;
    }

    Ok(())
}

#[allow(dead_code, unused_variables, unused_imports)]

fn handle_partnotes(cx: &CoreContext, partidx: usize, partnotes: &VecPartNotes) -> Result<(), Box<dyn Error>> {
    for item in partnotes.windows(2) {
        let left = item[0];
        let right = item[1];
        // println!("left:{:?}, right:{:?}", left, right);

        match (left, right) {
            ((Some(left_id), left_pos, left_sysid), (Some(right_id), right_pos, right_sysid)) if left_sysid == right_sysid => {
                handle_tie_pair(cx, left_id, right_id, left_pos, right_pos, left_sysid, right_sysid)?;
            }
            ((Some(left_id), left_pos, left_sysid), (Some(right_id), right_pos, right_sysid)) => {
                handle_tie_pair(cx, left_id, right_id, left_pos, right_pos, left_sysid, right_sysid)?;
            }
            ((Some(left_id), left_pos, left_sysid), (None, right_pos, right_sysid)) => {
                //
            }
            ((None, left_pos, left_sysid), (Some(right_id), right_pos, right_sysid)) => {
                //
            }
            ((None, _, _), (None, _, _)) => {
                // Both are None, no action needed
            }
        }

        // if let (Some(left_note_id), Some(right_note_id)) = (left.0, right.0) {
        //     if left.1 == right.1 && left.2 == right.2 {
        //         // Tie detected
        //         // Here you would handle the tie logic, e.g., updating the complex or note items
        //         println!(
        //             "Tie detected between note {} and note {} at position {} in system {}",
        //             left_note_id, right_note_id, left.1, left.2
        //         );
        //     }
        // }
    }

    Ok(())
}

fn handle_tie_pair(
    cx: &CoreContext,
    left_id: usize,
    right_id: usize,
    left_pos: SysitemPosition,
    right_pos: SysitemPosition,
    left_sysid: SysitemId,
    right_sysid: SysitemId,
) -> Result<(), Box<dyn Error>> {
    // Here you would implement the logic to handle a tie pair
    // For example, updating the complex or note items
    println!("-----------------------------------");
    if left_sysid != right_sysid {
        println!("Different SYSITEMs: Left and right sysitem IDs do not match: {} != {}", left_sysid, right_sysid);
    }

    println!("Check handling tie pair between note {} and note {} ", left_id, right_id);

    let tiesfrom = cx.map_noteid_tiesfrom.borrow();
    let notes = cx.notes.borrow();

    if tiesfrom.contains_key(&left_id) {
        println!("Left id has tie from!");

        let levels_from = tiesfrom.get(&left_id).unwrap();
        let left_note = &notes[left_id];
        let right_note = &notes[right_id];

        if (left_pos + left_note.position + (left_note.duration as usize)) < right_pos + right_note.position {
            println!("Left note ends before right note starts, no tie possible.");
            // store unresolved tie from left_id
            return Ok(());
        }

        if let Some(levels_to) = right_note.get_head_levels() {
            for level_from in levels_from {
                match level_from {
                    TieFrom::Level(left_level) => {
                        if levels_to.contains(left_level) {
                            println!("Tie detected between note {} and note {} at level {}", left_id, right_id, left_level);
                            // Store resolved from left_id and to right_id
                            cx.map_noteid_resolvedtiesfrom.borrow_mut().entry(left_id).or_default().push(ResolvedTieFrom::Resolved(*left_level));
                            cx.map_noteid_resolvedtiesto.borrow_mut().entry(right_id).or_default().push(ResolvedTieTo::Level(*left_level));
                        } else {
                            println!("No tie found for level: {}", left_level);
                            cx.map_noteid_resolvedtiesfrom.borrow_mut().entry(left_id).or_default().push(ResolvedTieFrom::Unresolved(*left_level));
                        }
                    }
                }
            }
        }
    } else {
        println!("Nothing to see here");
    }

    Ok(())
}
