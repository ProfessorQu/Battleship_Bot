//! This is the module for all the place functions
//! 
//! This module contains all the functions to place boats,
//! It's important that they're functions because otherwise `Battleship::reset` wouldn't work.

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

fn valid_boat_pos(boats: &BoatMap, boat: Boat, get_boat_pos: fn(Boat) -> (bool, Pos)) -> (bool, Pos) {
    let mut boat_pos = get_boat_pos(boat);

    while overlaps(boats, boat, boat_pos.0, boat_pos.1) {
        boat_pos = get_boat_pos(boat);
    }

    boat_pos
}

fn random_boat_pos(boat: Boat) -> (bool, Pos) {
    let horizontal: bool = rand::random();

    let (x_range, y_range) = if horizontal {
        (
            0..(NUM_COLS - boat.length()),
            0..NUM_ROWS
        )
    } else {
        (
            0..NUM_COLS,
            0..(NUM_ROWS - boat.length())
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
/// use battleship_bot::{shoot, place};
/// 
/// let mut battleship = Battleship::new(
///     place::random,
///     place::random,
/// 
///     shoot::random,
///     shoot::random
/// );
/// 
/// let recording = battleship.play_and_record_game();
/// 
/// println!("{} won!", recording.winner);
/// ```
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
            if rand::random() { rng.gen_range(0..2) } else { rng.gen_range(NUM_ROWS - 2..NUM_ROWS) }
        )
    } else {
        (
            if rand::random() { rng.gen_range(0..2) } else { rng.gen_range(NUM_COLS - 2..NUM_COLS) },
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
/// use battleship_bot::{shoot, place};
/// 
/// let mut battleship = Battleship::new(
///     place::sides,
///     place::sides,
/// 
///     shoot::random,
///     shoot::random
/// );
/// 
/// let recording = battleship.play_and_record_game();
/// 
/// println!("{} won!", recording.winner);
/// ```
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

    let (x_min, mut x_max) = match boat {
        Boat::Destroyer => (0, NUM_COLS / 2),
        Boat::Submarine => (NUM_COLS / 2 + 1, NUM_COLS),
        Boat::Cruiser => (0, NUM_COLS / 2),
        Boat::Battleship => (NUM_COLS / 2, NUM_COLS),
        _ => unreachable!()
    };

    if horizontal {
        x_max -= boat.length();
    }

    let (y_min, mut y_max) = match  boat {
        Boat::Destroyer => (0, NUM_ROWS / 2),
        Boat::Submarine => (0, NUM_ROWS / 2),
        Boat::Cruiser => (NUM_ROWS / 2 + 1, NUM_ROWS),
        Boat::Battleship => (NUM_ROWS / 2, NUM_ROWS),
        _ => unreachable!()
    };

    if !horizontal {
        y_max -= boat.length();
    }

    let (x, y) = (
        rng.gen_range(x_min..x_max),
        rng.gen_range(y_min..y_max)
    );

    (horizontal, pos!(x, y))
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
/// use battleship_bot::{shoot, place};
/// 
/// let mut battleship = Battleship::new(
///     place::spread,
///     place::spread,
/// 
///     shoot::random,
///     shoot::random
/// );
/// 
/// let recording = battleship.play_and_record_game();
/// 
/// println!("{} won!", recording.winner);
/// ```
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
            NUM_COLS / 4..NUM_COLS * 3 / 4 + 1 - length,
            NUM_ROWS / 4..NUM_ROWS * 3 / 4 + 1
        )
    } else {
        (
            NUM_COLS / 4..NUM_COLS * 3 / 4 + 1,
            NUM_ROWS / 4..NUM_ROWS * 3 / 4 + 1 - length
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
/// use battleship_bot::{shoot, place};
/// 
/// let mut battleship = Battleship::new(
///     place::cluster,
///     place::cluster,
/// 
///     shoot::random,
///     shoot::random
/// );
/// 
/// let recording = battleship.play_and_record_game();
/// 
/// println!("{} won!", recording.winner);
/// ```
pub fn cluster() -> BoatMap {
    let mut boats = [[Boat::Empty; NUM_ROWS]; NUM_COLS];

    for boat in BOATS {
        let (horizontal, pos) = valid_boat_pos(&boats, boat, cluster_boat_pos);
        place_boat(&mut boats, boat, horizontal, pos);
    }

    boats
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one_boat(boat: Boat, pos: Pos, horizontal: bool) -> BoatMap {
        let mut boats = [[Boat::Empty; NUM_ROWS]; NUM_COLS];

        place_boat(&mut boats, boat, horizontal, pos);

        boats
    }

    fn is_horizontal(boats: &BoatMap, x: usize, y: usize) -> bool {
        let boat = boats[x][y];
        let mut horizontal = false;
        if x > 0 {
            horizontal = boats[x - 1][y] == boat;
        }
        if x < NUM_COLS - 1 {
            horizontal = horizontal || boats[x + 1][y] == boat;
        }

        horizontal
    }

    fn is_vertical(boats: &BoatMap, x: usize, y: usize) -> bool {
        let boat = boats[x][y];
        let mut vertical = false;
        if y > 0 {
            vertical = boats[x][y - 1] == boat;
        }
        if y < NUM_ROWS - 1 {
            vertical = vertical || boats[x][y + 1] == boat;
        }

        vertical
    }

    #[test]
    fn test_overlaps_horizontal() {
        let boats = one_boat(Boat::Destroyer, pos!(1, 0), false);

        assert!(overlaps(&boats, Boat::Destroyer, true, pos!(0, 0)));
        assert!(!overlaps(&boats, Boat::Destroyer, false, pos!(0, 0)));
        assert!(overlaps(&boats, Boat::Destroyer, true, pos!(0, 1)));
        assert!(!overlaps(&boats, Boat::Destroyer, true, pos!(0, 2)));

        let boats = one_boat(Boat::Destroyer, pos!(2, 0), false);

        assert!(!overlaps(&boats, Boat::Destroyer, true, pos!(0, 0)));
        for boat in BOATS {
            if boat == Boat::Destroyer {
                continue
            }

            assert!(overlaps(&boats, boat, true, pos!(0, 0)));

            assert!(!overlaps(&boats, boat, false, pos!(0, 0)));
        }
    }

    #[test]
    fn test_overlaps_vertical() {
        let boats = one_boat(Boat::Destroyer, pos!(0, 1), true);

        assert!(overlaps(&boats, Boat::Destroyer, false, pos!(0, 0)));
        assert!(!overlaps(&boats, Boat::Destroyer, true, pos!(0, 0)));
        assert!(overlaps(&boats, Boat::Destroyer, false, pos!(1, 0)));
        assert!(!overlaps(&boats, Boat::Destroyer, false, pos!(2, 0)));

        let boats = one_boat(Boat::Destroyer, pos!(0, 2), true);

        assert!(!overlaps(&boats, Boat::Destroyer, false, pos!(0, 0)));
        for boat in BOATS {
            if boat == Boat::Destroyer {
                continue
            }

            assert!(overlaps(&boats, boat, false, pos!(0, 0)));

            assert!(!overlaps(&boats, boat, true, pos!(0, 0)));
        }
    }

    fn test_sides() {
        let boats = sides();

        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if boats[x][y] != Boat::Empty {
                    if (2..=7).contains(&x) {
                        assert!(is_horizontal(&boats, x, y));
                    }
                    if (2..=7).contains(&y) {
                        assert!(is_vertical(&boats, x, y));
                    }
                }
            }
        }
    }

    #[test]
    fn test_sides_100() {
        for _ in 0..100 {
            test_sides();
        }
    }

    fn test_spread() {
        let boats = spread();

        for (x, row) in boats.iter().enumerate() {
            for (y, boat) in row.iter().enumerate() {
                match boat {
                    Boat::Destroyer => {
                        assert!(x <= 5);
                        assert!(y <= 5);
                    }
                    Boat::Submarine => {
                        assert!(x >= 6);
                        assert!(y <= 5);
                    }
                    Boat::Cruiser => {
                        assert!(x <= 5);
                        assert!(y >= 6);
                    }
                    Boat::Battleship => {
                        assert!(x >= 5);
                        assert!(y >= 5);
                    }
                    _ => ()
                }
            }
        }
    }

    #[test]
    fn test_spread_100() {
        for _ in 0..100 {
            test_spread();
        }
    }

    fn test_cluster() {
        let boats = cluster();

        for (x, row) in boats.iter().enumerate() {
            for (y, boat) in row.iter().enumerate() {
                if *boat != Boat::Empty {
                    assert!((2..=7).contains(&x));
                    assert!((2..=7).contains(&y));
                }
            }
        }
    }

    #[test]
    fn test_cluster_100() {
        for _ in 0..100 {
            test_cluster();
        }
    }
}
