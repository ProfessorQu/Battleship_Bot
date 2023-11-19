use rand::Rng;

use crate::pos;
use crate::battleship::pos::Pos;
use crate::battleship::boat::{Boat, BOATS};
use crate::battleship::constants::{NUM_COLS, NUM_ROWS, BoatMap};

fn overlaps(
    boats: &BoatMap,
    boat: Boat,
    horizontal: bool, pos: Pos
) -> bool {
    let mut overlaps = false;

    if horizontal {
        for x_off in 0..boat.length() {
            if boats[pos.x + x_off][pos.y].has_some() {
                overlaps = true;
                break;
            }
        }
    }
    else {
        for y_off in 0..boat.length() {
            if boats[pos.x][pos.y + y_off].has_some() {
                overlaps = true;
                break;
            }
        }
    }

    overlaps
}

fn random_boat_pos(boat: Boat) -> (bool, Pos) {
    let horizontal: bool = rand::random();
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

fn place_boat(boats: &mut BoatMap, boat: Boat, horizontal: bool, pos: Pos) {
    if horizontal {
        for x_off in 0..boat.length() {
            debug_assert!(boats[pos.x + x_off][pos.y].is_empty());
            boats[pos.x + x_off][pos.y] = boat;
        }
    }
    else {
        for y_off in 0..boat.length() {
            debug_assert!(boats[pos.x][pos.y + y_off].is_empty());
            boats[pos.x][pos.y + y_off] = boat;
        }
    }
}

pub fn place_boats_random() -> BoatMap {
    let mut boats = [[Boat::Empty; NUM_ROWS]; NUM_COLS];

    for boat in BOATS {
        let (horizontal, pos) = valid_boat_pos(&boats, boat, random_boat_pos);
        place_boat(&mut boats, boat, horizontal, pos);
    }

    boats
}

fn side_boat_pos(boat: Boat) -> (bool, Pos) {
    let horizontal: bool = rand::random();

    let mut rng = rand::thread_rng();

    let (x, y) = if horizontal {
        (
            rng.gen_range(0..NUM_COLS - boat.length()),
            if rand::random() { rng.gen_range(0..2) } else { NUM_ROWS - 1 }
        )
    } else {
        (
            if rand::random() { rng.gen_range(0..2) } else { NUM_COLS - 1 },
            rng.gen_range(0..NUM_ROWS - boat.length())
        )
    };

    (horizontal, pos!(x, y))
}

pub fn place_boats_sides() -> BoatMap {
    let mut boats = [[Boat::Empty; NUM_ROWS]; NUM_COLS];

    for boat in BOATS {
        let (horizontal, pos) = valid_boat_pos(&boats, boat, side_boat_pos);
        place_boat(&mut boats, boat, horizontal, pos);
    }

    boats
}

fn spread_boat_pos(boat: Boat) -> (bool, Pos) {
    if matches!(boat, Boat::Carrier) {
        return random_boat_pos(boat);
    }

    let horizontal: bool = rand::random();

    let mut rng = rand::thread_rng();

    let length = boat.length();
    let id = boat as usize;

    let x_range = if id % 2 == 1 && horizontal {
        0..NUM_COLS / 2 - length
    } else if id % 2 == 1 {
        0..NUM_COLS / 2
    } else if id % 2 == 0 && horizontal {
        (NUM_COLS / 2 - 1)..NUM_COLS - 1 - length
    } else {
        (NUM_COLS / 2)..NUM_COLS - 1
    };

    let y_range = if id <= 2 && horizontal {
        0..NUM_ROWS / 2
    } else if id <= 2 {
        0..NUM_ROWS / 2 - length
    } else if id > 2 && horizontal {
        (NUM_ROWS / 2)..NUM_ROWS - 1
    } else {
        (NUM_ROWS / 2 - 1)..NUM_ROWS - 1 - length
    };

    let (x, y) = (
        rng.gen_range(x_range),
        rng.gen_range(y_range)
    );

    (horizontal, pos!(x, y))
}

fn valid_boat_pos(boats: &BoatMap, boat: Boat, get_boat_pos: fn(Boat) -> (bool, Pos)) -> (bool, Pos) {
    let mut boat_pos = get_boat_pos(boat);

    while overlaps(boats, boat, boat_pos.0, boat_pos.1) {
        boat_pos = get_boat_pos(boat);
    }

    boat_pos
}

pub fn place_boats_spread() -> BoatMap {
    let mut boats = [[Boat::Empty; NUM_ROWS]; NUM_COLS];

    for boat in BOATS {
        let (horizontal, pos) = valid_boat_pos(&boats, boat, spread_boat_pos);
        place_boat(&mut boats, boat, horizontal, pos);
    }

    boats
}
