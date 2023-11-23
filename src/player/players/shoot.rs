//! All the shoot functions
//! 
//! This module contains all the functions to shoot at the boats.
//! All functions take a 2d vector to see what shots have been taken and return a new position to fire.
//! If you want to implement your own place function, it has to take a [`ShotMap`] and a [`Pos`] and return a [`Pos`].
//! 
//! # Example
//! ```rust
//! use battleship_bot::*;
//! use rand::random;
//! 
//! fn shoot(_last_pos: Pos, shots: ShotMap) -> Pos {
//!     let mut shot = random();
//!     while !valid_shot(shots, shot) {
//!         shot = random();
//!     }
//! 
//!     shot
//! }
//! 
//! let mut game = Battleship::new(
//!     place::random,
//!     place::random,
//! 
//!     shoot,
//!     shoot::random
//! );
//! 
//! println!("{} won", game.play_and_record_game().winner);
//! ```

use rand::seq::SliceRandom;

use crate::battleship::position::Pos;
use crate::player::destroy::{random_destroy, destroy};
use crate::player::utils::get_hits;
use crate::pos;
use crate::battleship::boat::{BOATS, Boat};
use crate::battleship::constants::{NUM_COLS, NUM_ROWS, ShotMap};

/// Check if pos is a valid position for a shot in shots
/// 
/// Checks if pos is in range of the board and the position isn't shot yet.
/// 
/// # Example
/// ```rust
/// use battleship_bot::*;
/// 
/// let mut shots = [[None; 10]; 10];
/// 
/// assert!(valid_shot(shots, pos!(0, 0)));
/// assert!(!valid_shot(shots, pos!(10, 0)));
/// assert!(!valid_shot(shots, pos!(10, 10)));
/// 
/// shots[3][5] = Some(Shot::Miss);
/// assert!(!valid_shot(shots, pos!(3, 5)));
/// ```
pub fn valid_shot(shots: ShotMap, pos: Pos) -> bool  {
    pos.x < NUM_COLS &&
    pos.y < NUM_ROWS &&
    shots[pos.x][pos.y].is_none()
}

fn random_find(shots: ShotMap) -> Pos {
    let mut shot = rand::random();

    while !valid_shot(shots, shot) {
        shot = rand::random();
    }

    shot
}

/// Shoots completely randomly
/// 
/// Shoot spaces that aren't shot randomly
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
/// let (p1_wins, p2_wins) = battleship.play_games(1_000);
/// 
/// // This is true because the starting player has a small advantage
/// assert!(p1_wins > p2_wins);
/// ```
pub fn random(_: Pos, shots: ShotMap) -> Pos {
    random_find(shots)
}

/// Shoots completely randomly until a ship is hit, then focuses on it
/// 
/// Shoot spaces that aren't shot randomly.
/// Until it hits a ship, then shoot around it to find the rest.
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
///     shoot::random_and_random_destroy
/// );
/// 
/// let (p1_wins, p2_wins) = battleship.play_games(1_000);
/// 
/// assert!(p2_wins > p1_wins);
/// ```
pub fn random_and_random_destroy(_: Pos, shots: ShotMap) -> Pos {
    if let Some(pos) = random_destroy(shots) {
        pos
    } else {
        random_find(shots)
    }
}

/// Shoots completely randomly until a ship is hit, then destroys it smarter than random_and_random_destroy
/// 
/// Shoot spaces that aren't shot randomly.
/// Until it hits a ship, then shoot around it to find the rest.
/// But also account for the fact that ships are a line, so don't shoot spaces to the sides.
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
///     shoot::random_and_random_destroy,
///     shoot::random_and_destroy
/// );
/// 
/// let (p1_wins, p2_wins) = battleship.play_games(1_000);
/// 
/// assert!(p2_wins > p1_wins);
/// ```
pub fn random_and_destroy(_: Pos, shots: ShotMap) -> Pos {
    if let Some(pos) = destroy(shots) {
        pos
    } else {
        random_find(shots)
    }
}

fn grid_find(shots: ShotMap, last_pos: Pos) -> Pos {
    if shots[0][0].is_none() {
        return pos!(0, 0)
    }

    let mut min_len = 6;

    for boat in BOATS {
        let hits = get_hits(shots);
        let hits = hits
            .iter()
            .filter(|shot| shot.0 == boat);
        let hits_len = hits.clone().count();

        let boat_len = boat.length();

        if hits_len < boat_len && boat_len < min_len {
            min_len = boat_len;
        }
    }

    let mut position = last_pos;

    let mut has_reset = false;
    while !valid_shot(shots, position) {
        position.x += min_len;
        
        if position.x >= NUM_COLS {
            position.y += 1;

            position.x %= NUM_COLS;
            if position.y % 2 == 1 && position.x == 0 {
                position.x += 1;
            } else if position.y % 2 == 0 && position.x == 1 {
                position.x -= 1;
            }
        }

        if position.y >= NUM_ROWS {
            if has_reset {
                position = random_find(shots);
            } else {
                position = pos!(0, 0);
                has_reset = true;
            }
        }
    }

    position
}

/// Shoots in a grid until it finds a ship and destroy it
/// 
/// Shoot in a grid pattern until it finds a ship, then destroy it.
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
///     shoot::random_and_destroy,
///     shoot::grid_and_destroy
/// );
/// 
/// let (p1_wins, p2_wins) = battleship.play_games(1_000);
/// 
/// assert!(p2_wins > p1_wins);
/// ```
pub fn grid_and_destroy(last_pos: Pos, shots: ShotMap) -> Pos {
    if let Some(pos) = destroy(shots) {
        pos
    } else {
        grid_find(shots, last_pos)
    }
}

fn update_heatmap(heatmap: &mut [[usize; NUM_ROWS]; NUM_COLS], shots: ShotMap, boat: Boat, horizontal: bool, pos: Pos) {
    let mut overlaps = false;

    if horizontal {
        for x_off in 0..boat.length() {
            if shots[pos.x + x_off][pos.y].is_some() {
                overlaps = true;
                break;
            }
        }
    } else {
        for y_off in 0..boat.length() {
            if shots[pos.x][pos.y + y_off].is_some() {
                overlaps = true;
                break;
            }
        }
    }

    if overlaps {
        return
    }

    if horizontal {
        for x_off in 0..boat.length() {
            heatmap[pos.x + x_off][pos.y] += 1;
        }
    } else {
        for y_off in 0..boat.length() {
            heatmap[pos.x][pos.y + y_off] += 1;
        }
    }
}

fn create_heatmap(shots: ShotMap) -> [[usize; NUM_ROWS]; NUM_COLS] {
    let mut heatmap = [[0; NUM_ROWS]; NUM_COLS];

    for boat in BOATS {
        for x in 0..=NUM_COLS - boat.length() {
            for y in 0..NUM_ROWS {
                update_heatmap(&mut heatmap, shots, boat, true, pos!(x, y));
            }
        }

        for x in 0..NUM_COLS {
            for y in 0..=NUM_ROWS - boat.length() {
                update_heatmap(&mut heatmap, shots, boat, false, pos!(x, y));
            }
        }
    }

    heatmap
}

fn heatmap_find(shots: ShotMap) -> Pos {
    let heatmap = create_heatmap(shots);

    let max = heatmap
        .iter().map(
            |row| row.iter().max().expect("No items in row")
        ).max().expect("No items in heatmap");

    let mut possible_positions = vec![];

    for (x, row) in heatmap.iter().enumerate() {
        for (y, heat) in row.iter().enumerate() {
            if heat == max {
                possible_positions.push(pos!(x, y));
            }
        }
    }

    let mut pos = *possible_positions
        .choose(&mut rand::thread_rng())
        .expect("Failed to choose random position");

    while !valid_shot(shots, pos) {
        pos = *possible_positions
            .choose(&mut rand::thread_rng())
            .expect("Failed to choose random position");
    }

    pos
}

/// Creates a heatmap for the boats and shoots the highest heat
/// 
/// Creates a heatmap using all possible positions the boats can be in, then shoots one of the highest ones.
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
///     shoot::grid_and_destroy,
///     shoot::heatmap_and_destroy
/// );
/// 
/// let (p1_wins, p2_wins) = battleship.play_games(1_000);
/// 
/// assert!(p2_wins > p1_wins);
/// ```
pub fn heatmap_and_destroy(_: Pos, shots: ShotMap) -> Pos {
    if let Some(pos) = destroy(shots) {
        pos
    } else {
        heatmap_find(shots)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::Shot;

    use super::*;

    #[test]
    fn test_valid_shot() {
        let mut shots = [[None; 10]; 10];

        for x in 0..10 {
            for y in 0..10 {
                assert!(valid_shot(shots, pos!(x, y)));
            }
        }

        assert!(!valid_shot(shots, pos!(10, 0)));
        assert!(!valid_shot(shots, pos!(0, 10)));
        assert!(!valid_shot(shots, pos!(10, 10)));

        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let x = rng.gen_range(0..10);
            let y = rng.gen_range(0..10);

            if rand::random() {
                shots[x][y] = Some(Shot::Miss);
            } else {
                shots[x][y] = Some(Shot::Hit(Boat::Destroyer));
            }

            assert!(!valid_shot(shots, pos!(x, y)));
        }
    }

    #[test]
    fn test_random_and_random_destroy() {
        let mut shots = [[None; NUM_ROWS]; NUM_COLS];

        shots[1][1] = Some(Shot::Hit(Boat::Cruiser));
        shots[1][2] = Some(Shot::Hit(Boat::Cruiser));
        
        let possible = [
            pos!(1, 0),
            pos!(1, 3),
            pos!(0, 1),
            pos!(0, 2),
            pos!(2, 1),
            pos!(2, 2)
        ];

        for _ in 0..100 {
            let shot = random_and_random_destroy(pos!(0, 0), shots);

            assert!(possible.contains(&shot));
        }
    }

    #[test]
    fn test_random_and_destroy() {
        let mut shots = [[None; NUM_ROWS]; NUM_COLS];

        shots[1][1] = Some(Shot::Hit(Boat::Cruiser));
        shots[1][2] = Some(Shot::Hit(Boat::Cruiser));
        
        let possible = [
            pos!(1, 0),
            pos!(1, 3),
        ];

        for _ in 0..100 {
            let shot = random_and_destroy(pos!(0, 0), shots);

            assert!(possible.contains(&shot));
        }
    }

    #[test]
    fn test_grid_and_destroy() {
        let mut shots = [[None; NUM_ROWS]; NUM_COLS];

        assert!(grid_and_destroy(pos!(0, 0), shots) == pos!(0, 0));

        shots[0][0]= Some(Shot::Hit(Boat::Destroyer));

        let shot = grid_and_destroy(pos!(0, 0), shots);
        shots[shot.x][shot.y] = Some(Shot::Hit(Boat::Destroyer));

        assert!(grid_and_destroy(pos!(0, 0), shots) == pos!(3, 0));
    }

    #[test]
    fn test_grid_and_destroy2() {
        let mut shots = [[None; NUM_ROWS]; NUM_COLS];

        assert!(grid_and_destroy(pos!(0, 0), shots) == pos!(0, 0));
        shots[0][0] = Some(Shot::Miss);

        assert!(grid_and_destroy(pos!(0, 0), shots) == pos!(2, 0));
        shots[2][0] = Some(Shot::Miss);

        assert!(grid_and_destroy(pos!(0, 0), shots) == pos!(4, 0));
    }

    #[test]
    fn test_create_heatmap() {
        let mut shots = [[None; NUM_ROWS]; NUM_COLS];

        let heatmap = create_heatmap(shots);

        assert!(heatmap[0][0] == 10);
        assert!(heatmap[0][NUM_ROWS - 1] == 10);
        assert!(heatmap[NUM_COLS - 1][0] == 10);
        assert!(heatmap[NUM_COLS - 1][NUM_ROWS - 1] == 10);

        for col in heatmap.iter().take(NUM_COLS - 1).skip(1) {
            for heat in col.iter() {
                assert!(*heat > 10);
            }
        }

        assert!(heatmap[0][1] == heatmap[1][0]);

        shots[0][0] = Some(Shot::Miss);
        let heatmap = create_heatmap(shots);
        assert!(heatmap[0][1] == 10);

        shots[0][2] = Some(Shot::Miss);
        let heatmap = create_heatmap(shots);
        assert!(heatmap[0][1] == 5);
    }

    #[test]
    fn test_heatmap_and_destroy() {
        let mut shots = [[None; NUM_ROWS]; NUM_COLS];

        let possible = [
            pos!(4, 4),
            pos!(4, 5),
            pos!(5, 4),
            pos!(5, 5),
        ];

        assert!(possible.contains(&heatmap_and_destroy(pos!(0, 0), shots)));

        shots[4][4] = Some(Shot::Miss);
        assert!(heatmap_and_destroy(pos!(0, 0), shots) == pos!(5, 5));
    }
}
