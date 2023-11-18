use std::vec;

use rand::seq::SliceRandom;

use crate::{battleship::{constants::{NUM_ROWS, NUM_COLS, OFFSETS}, game::Shot, boat::{Boat, BOATS}, Pos}, pos};

const LENGTHS: [usize; 5] = [2, 3, 3, 4, 5];

pub fn length(boat: Boat) -> usize {
    LENGTHS[boat as usize - 1]
}

pub fn valid_boat_pos(
    boats: &[[Boat; NUM_ROWS]; NUM_COLS],
    boat: Boat,
    horizontal: bool, pos: Pos
) -> bool {
    let mut valid_position = true;

    if horizontal {
        for x_off in 0..length(boat) {
            if boats[pos.x + x_off][pos.y].has_some() {
                valid_position = false;
                break;
            }
        }
    }
    else {
        for y_off in 0..length(boat) {
            if boats[pos.x][pos.y + y_off].has_some() {
                valid_position = false;
                break;
            }
        }
    }

    valid_position
}

pub fn valid_shot(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS], pos: Pos) -> bool  {
    shots[pos.x][pos.y].is_none()
}

pub fn valid_shot_any(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS], x: i32, y: i32) -> bool {
    x >= 0 && x < NUM_COLS as i32 &&
    y >= 0 && y < NUM_ROWS as i32 &&
    shots[x as usize][y as usize].is_none()
}

fn add_valid_position_with_offset(positions: &mut Vec<Pos>, shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS], x: i32, y: i32) {
    if valid_shot_any(shots, x, y) {
        positions.push(pos!(
            x as usize,
            y as usize
        ));
    }
}

fn random_offset_shoot_pos(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS], boat_hits_vec: Vec<Pos>) -> Option<Pos> {
    let pos = if boat_hits_vec.len() == 1 {
        boat_hits_vec.first().copied().expect("No hits")
    }
    else {
        let min_pos = boat_hits_vec
            .first().copied().expect("No hits");
        let max_pos = boat_hits_vec
            .last().copied().expect("No hits");

        [min_pos, max_pos]
            .choose(&mut rand::thread_rng())
            .copied().expect("No hits")
    };
    let mut positions = vec![];

    let (x, y) = (pos.x as i32, pos.y as i32);

    for offset in OFFSETS {
        add_valid_position_with_offset(&mut positions, shots, x + offset.0, y + offset.1);
    }

    positions.choose(&mut rand::thread_rng()).copied()
}

fn offset_shoot_pos(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS], boat_hits_vec: Vec<Pos>) -> Option<Pos> {
    let min_pos = boat_hits_vec
        .first().copied().expect("No boats");
    let max_pos = boat_hits_vec
        .last().copied().expect("No boats");

    let horizontal = max_pos.x - min_pos.x != 0;

    let mut positions = vec![];

    if horizontal {
        if valid_shot_any(shots, min_pos.x as i32 - 1, min_pos.y as i32) {
            positions.push(pos!(min_pos.x - 1, min_pos.y));
        }
        if valid_shot_any(shots, max_pos.x as i32 + 1, max_pos.y as i32) {
            positions.push(pos!(max_pos.x + 1, max_pos.y));
        }
    } else {
        if valid_shot_any(shots, min_pos.x as i32, min_pos.y as i32 - 1) {
            positions.push(pos!(min_pos.x, min_pos.y - 1));
        }
        if valid_shot_any(shots, max_pos.x as i32, max_pos.y as i32 + 1) {
            positions.push(pos!(max_pos.x, max_pos.y + 1));
        }
    }

    positions.choose(&mut rand::thread_rng()).copied()
}

fn get_hits(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> Vec<(Boat, usize, usize)> {
    let mut hits = vec![];

    for (x, column) in shots.iter().enumerate() {
        for (y, shot) in column.iter().enumerate() {
            if let Some(Shot::Hit(boat)) = shot {
                hits.push((*boat, x, y));
            }
        }
    }

    hits
}

pub fn random_destroy(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> Option<Pos> {
    let hits = get_hits(shots);

    for boat in BOATS {
        let boat_hits = hits
            .iter()
            .filter_map(|hit| if hit.0 == boat {
                Some(pos!(hit.1, hit.2))
            } else {
                None
            });

        let hits_len = boat_hits.clone().count();

        if hits_len > 0 && hits_len < length(boat) {
            let boat_hits_vec: Vec<Pos> = boat_hits.collect();

            return random_offset_shoot_pos(shots, boat_hits_vec)
        }
    }

    None
}

pub fn destroy(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> Option<Pos> {
    let hits = get_hits(shots);

    for boat in BOATS {
        let boat_hits = hits
            .iter()
            .filter_map(|hit| if hit.0 == boat {
                Some(pos!(hit.1, hit.2))
            } else {
                None
            });

        let hits_len = boat_hits.clone().count();

        if hits_len > 0 && hits_len < length(boat) {
            let boat_hits_vec: Vec<Pos> = boat_hits.collect();

            if hits_len > 1 {
                return offset_shoot_pos(shots, boat_hits_vec);
            } else {
                return random_offset_shoot_pos(shots, boat_hits_vec)
            }
        }
    }

    None
}
