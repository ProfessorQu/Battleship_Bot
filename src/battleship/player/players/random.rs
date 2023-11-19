use rand::Rng;

use crate::{battleship::{constants::{NUM_COLS, NUM_ROWS, BoatMap, ShotMap}, boat::{Boat, BOATS}, Pos, player::destroy::{valid_shot, random_destroy, destroy}, Player}, pos};

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

fn random_valid_boat_pos(boats: &BoatMap, boat: Boat) -> (bool, Pos) {
    let (mut horizontal, mut pos) = random_boat_pos(boat);

    while overlaps(boats, boat, horizontal, pos) {
        (horizontal, pos) = random_boat_pos(boat);
    }

    (horizontal, pos)
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
        let (horizontal, pos) = random_valid_boat_pos(&boats, boat);
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

fn side_valid_boat_pos(boats: &BoatMap, boat: Boat) -> (bool, Pos) {
    let (mut horizontal, mut pos) = side_boat_pos(boat);

    while overlaps(boats, boat, horizontal, pos) {
        (horizontal, pos) = side_boat_pos(boat);
    }

    (horizontal, pos)
}

pub fn place_boats_sides() -> BoatMap {
    let mut boats = [[Boat::Empty; NUM_ROWS]; NUM_COLS];

    for boat in BOATS {
        let (horizontal, pos) = side_valid_boat_pos(&boats, boat);
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

fn spread_valid_boat_pos(boats: &BoatMap, boat: Boat) -> (bool, Pos) {
    let (mut horizontal, mut pos) = spread_boat_pos(boat);

    while overlaps(boats, boat, horizontal, pos) {
        (horizontal, pos) = spread_boat_pos(boat);
    }

    (horizontal, pos)
}

pub fn place_boats_spread() -> BoatMap {
    let mut boats = [[Boat::Empty; NUM_ROWS]; NUM_COLS];

    for boat in BOATS {
        let (horizontal, pos) = spread_valid_boat_pos(&boats, boat);
        place_boat(&mut boats, boat, horizontal, pos);
    }

    boats
}

pub fn find(shots: ShotMap) -> Pos {
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

pub fn shoot(_player: Player, shots: ShotMap) -> Pos {
    find(shots)
}

pub fn find_and_random_destroy(_player: Player, shots: ShotMap) -> Pos {
    if let Some(pos) = random_destroy(shots) {
        pos
    } else {
        find(shots)
    }
}

pub fn find_and_destroy(_player: Player, shots: ShotMap) -> Pos {
    if let Some(pos) = destroy(shots) {
        pos
    } else {
        find(shots)
    }
}
