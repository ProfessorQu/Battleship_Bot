use std::vec;
use rand::seq::SliceRandom;

use crate::battleship::position::Pos;
use crate::pos;
use crate::battleship::boat::BOATS;
use crate::battleship::constants::{NUM_ROWS, NUM_COLS, OFFSETS, ShotMap};

use super::utils::get_hits;

pub fn valid_shot_any(shots: ShotMap, x: i32, y: i32) -> bool {
    x >= 0 && x < NUM_COLS as i32 &&
    y >= 0 && y < NUM_ROWS as i32 &&
    shots[x as usize][y as usize].is_none()
}

fn add_valid_position_with_offset(positions: &mut Vec<Pos>, shots: ShotMap, x: i32, y: i32) {
    if valid_shot_any(shots, x, y) {
        positions.push(pos!(
            x as usize,
            y as usize
        ));
    }
}

fn random_offset_shoot_pos(shots: ShotMap, boat_hits_vec: Vec<Pos>) -> Option<Pos> {
    let pos = if boat_hits_vec.len() == 1 {
        boat_hits_vec.first().copied().expect("No hits in boat_hits_vec")
    }
    else {
        let min_pos = boat_hits_vec
            .first().copied().expect("No hits in boat_hits_vec");
        let max_pos = boat_hits_vec
            .last().copied().expect("No hits in boat_hits_vec");

        [min_pos, max_pos]
            .choose(&mut rand::thread_rng())
            .copied().expect("No hits in boat_hits_vec")
    };

    let mut positions = vec![];

    let (x, y) = (pos.x as i32, pos.y as i32);

    for offset in OFFSETS {
        add_valid_position_with_offset(&mut positions, shots, x + offset.0, y + offset.1);
    }

    positions.choose(&mut rand::thread_rng()).copied()
}

fn offset_shoot_pos(shots: ShotMap, boat_hits_vec: Vec<Pos>) -> Option<Pos> {
    let min_pos = boat_hits_vec
        .first().copied().expect("No boats in boat_hits_vec");
    let max_pos = boat_hits_vec
        .last().copied().expect("No boats in boat_hits_vec");

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


pub fn random_destroy(shots: ShotMap) -> Option<Pos> {
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

        if hits_len == 0 || hits_len == boat.length() {
            continue
        }

        let boat_hits_vec: Vec<Pos> = boat_hits.collect();
        return random_offset_shoot_pos(shots, boat_hits_vec)
    }

    None
}

pub fn destroy(shots: ShotMap) -> Option<Pos> {
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

        if hits_len == 0 || hits_len == boat.length() {
            continue
        }

        let boat_hits_vec: Vec<Pos> = boat_hits.collect();

        if hits_len > 1 {
            return offset_shoot_pos(shots, boat_hits_vec);
        } else {
            return random_offset_shoot_pos(shots, boat_hits_vec)
        }
    }

    None
}
