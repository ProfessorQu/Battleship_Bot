use rand::{random, Rng};

use crate::{battleship::{constants::{NUM_COLS, NUM_ROWS}, game::Shot, boat::{Boat, BOATS}, Pos, player::destroy::{valid_boat_pos, valid_shot, random_destroy, destroy}}, pos};


fn random_boat_pos(boat: Boat) -> (bool, Pos) {
    let horizontal: bool = random();
    let length = boat.length();

    let (x_range, y_range) = if horizontal {
        (
            0..(NUM_COLS - length),
            0..NUM_ROWS
        )
    } else {
        (
            0..NUM_COLS,
            0..(NUM_ROWS - length)
        )
    };

    let mut rng = rand::thread_rng();
    
    (
        horizontal,
        pos!( 
            rng.gen_range(x_range),
            rng.gen_range(y_range)
        )
    )
}

fn random_valid_boat_pos(boats: &[[Boat; NUM_ROWS]; NUM_COLS], boat: Boat) -> (bool, Pos) {
    let (mut horizontal, mut pos) = random_boat_pos(boat);

    while !valid_boat_pos(boats, boat, horizontal, pos) {
        (horizontal, pos) = random_boat_pos(boat);
    }

    (horizontal, pos)
}

fn place_boat(boats: &mut [[Boat; NUM_ROWS]; NUM_COLS], boat: Boat) {
    let (horizontal, pos) = random_valid_boat_pos(boats, boat);

    if horizontal {
        for x_off in 0..boat.length() {
            boats[pos.x + x_off][pos.y] = boat;
        }
    }
    else {
        for y_off in 0..boat.length() {
            boats[pos.x][pos.y + y_off] = boat;
        }
    }
}

pub fn place() -> [[Boat; NUM_ROWS]; NUM_COLS] {
    let mut boats = [[Boat::Empty; NUM_ROWS]; NUM_COLS];

    for boat in BOATS {
        place_boat(&mut boats, boat);
    }

    boats
}

pub fn find(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> Pos {
    let mut rng = rand::thread_rng();

    let (mut x, mut y) = (
        rng.gen_range(0..NUM_COLS),
        rng.gen_range(0..NUM_ROWS),
    );

    while !valid_shot(shots, pos!(x, y)) {
        (x, y) = (
            rng.gen_range(0..NUM_COLS),
            rng.gen_range(0..NUM_ROWS),
        );
    }

    pos!(x, y)
}

pub fn find_and_destroy(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> Pos {
    if let Some(pos) = random_destroy(shots) {
        pos
    } else {
        find(shots)
    }
}

pub fn find_and_smart_destroy(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> Pos {
    if let Some(pos) = destroy(shots) {
        pos
    } else {
        find(shots)
    }
}
