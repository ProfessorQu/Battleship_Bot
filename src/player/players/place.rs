use rand::Rng;

use crate::pos;
use crate::battleship::position::Pos;
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

/// Place boats completely randomly
/// 
/// Place the boats with a random orientation and position
/// 
/// # Example
/// 
/// ```rust
/// use battleship_bot::Battleship;
/// use battleship_bot::players;
/// 
/// let mut battleship = Battleship::new(
///     players::place::random,
///     players::place::random,
/// 
///     players::shoot::random,
///     players::shoot::random
/// );
/// 
/// let recording = battleship.play_and_record_game();
/// 
/// println!("{} won!", recording.winner);
/// ````
pub fn random() -> BoatMap {
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

/// Place boats at the sides
/// 
/// Place the boats at a random position on the sides
/// with a possible offset of 1
/// 
/// # Example
/// 
/// ```rust
/// use battleship_bot::Battleship;
/// use battleship_bot::players;
/// 
/// let mut battleship = Battleship::new(
///     players::place::sides,
///     players::place::sides,
/// 
///     players::shoot::random,
///     players::shoot::random
/// );
/// 
/// let recording = battleship.play_and_record_game();
/// 
/// println!("{} won!", recording.winner);
/// ````
pub fn sides() -> BoatMap {
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

/// Place boats spread out
/// 
/// Place the boats spread out,
/// The Destroyer (1) will go in the top-left of the board,
/// The Submarine (2) will go in the top-right,
/// The Cruiser (3) will go in the bottom-left,
/// The Battleship (4) will go in the bottom-right,
/// And the Carrier (5) will go anywhere on the board
/// 
/// # Example
/// 
/// ```rust
/// use battleship_bot::Battleship;
/// use battleship_bot::players;
/// 
/// let mut battleship = Battleship::new(
///     players::place::spread,
///     players::place::spread,
/// 
///     players::shoot::random,
///     players::shoot::random
/// );
/// 
/// let recording = battleship.play_and_record_game();
/// 
/// println!("{} won!", recording.winner);
/// ````
pub fn spread() -> BoatMap {
    let mut boats = [[Boat::Empty; NUM_ROWS]; NUM_COLS];

    for boat in BOATS {
        let (horizontal, pos) = valid_boat_pos(&boats, boat, spread_boat_pos);
        place_boat(&mut boats, boat, horizontal, pos);
    }

    boats
}

fn cluster_boat_pos(boat: Boat) -> (bool, Pos) {
    let horizontal: bool = rand::random();
    let length = boat.length();

    let (x_range, y_range) = if horizontal {
        (
            NUM_COLS / 4 - 1..((NUM_COLS * 3) / 4 - length),
            NUM_ROWS / 4 - 1..NUM_ROWS * 3 / 4
        )
    } else {
        (
            NUM_COLS / 4 - 1..NUM_COLS * 3 / 4,
            NUM_ROWS / 4 - 1..((NUM_ROWS * 3) / 4 - length)
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

/// Place boats clustered
/// 
/// Place the boats all clustered together in the middle
/// 
/// # Example
/// 
/// ```rust
/// use battleship_bot::Battleship;
/// use battleship_bot::players;
/// 
/// let mut battleship = Battleship::new(
///     players::place::cluster,
///     players::place::cluster,
/// 
///     players::shoot::random,
///     players::shoot::random
/// );
/// 
/// let recording = battleship.play_and_record_game();
/// 
/// println!("{} won!", recording.winner);
/// ````
pub fn cluster() -> BoatMap {
    let mut boats = [[Boat::Empty; NUM_ROWS]; NUM_COLS];

    for boat in BOATS {
        let (horizontal, pos) = valid_boat_pos(&boats, boat, cluster_boat_pos);
        place_boat(&mut boats, boat, horizontal, pos);
    }

    boats
}
