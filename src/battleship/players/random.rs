use rand::{random, Rng};

use crate::battleship::{constants::{BOATS, NUM_COLS, NUM_ROWS, Boat}, game::Shot};

use super::utils::{valid_pos, valid_shot, random_focus, length};


fn random_boat_pos(boat: Boat) -> (bool, usize, usize) {
    let horizontal: bool = random();

    let (x_range, y_range) = if horizontal {
        (
            0..(NUM_COLS - length(boat)),
            0..NUM_ROWS
        )
    } else {
        (
            0..NUM_COLS,
            0..(NUM_ROWS - length(boat))
        )
    };

    let mut rng = rand::thread_rng();
    
    (
        horizontal,
        rng.gen_range(x_range),
        rng.gen_range(y_range)
    )
}

fn random_valid_boat_pos(boats: &[[Boat; NUM_ROWS]; NUM_COLS], boat: Boat) -> (bool, usize, usize) {
    let (mut horizontal, mut x, mut y) = random_boat_pos(boat);

    while !valid_pos(boats, boat, horizontal, x, y) {
        (horizontal, x, y) = random_boat_pos(boat);
    }

    (horizontal, x, y)
}

fn place_boat(boats: &mut [[Boat; NUM_ROWS]; NUM_COLS], boat: Boat) {
    let (horizontal, x, y) = random_valid_boat_pos(boats, boat);

    if horizontal {
        for x_off in 0..length(boat) {
            boats[x + x_off][y] = boat;
        }
    }
    else {
        for y_off in 0..length(boat) {
            boats[x][y + y_off] = boat;
        }
    }
}

pub fn place() -> [[Boat; NUM_ROWS]; NUM_COLS] {
    let mut boats = [[0; NUM_ROWS]; NUM_COLS];

    for boat in BOATS {
        place_boat(&mut boats, boat);
    }

    boats
}

pub fn shoot(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> (usize, usize) {
    let mut rng = rand::thread_rng();

    let (mut x, mut y) = (
        rng.gen_range(0..NUM_COLS),
        rng.gen_range(0..NUM_ROWS)
    );

    while !valid_shot(shots, x, y) {
        (x, y) = (
            rng.gen_range(0..NUM_COLS),
            rng.gen_range(0..NUM_ROWS)
        );
    }

    (x, y)
}

pub fn shoot_and_focus(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> (usize, usize) {
    if let Some(pos) = random_focus(shots) {
        pos
    } else {
        shoot(shots)
    }
}