use lazy_static::lazy_static;

use super::{boat::Boat, game::Shot, Player, Pos};
use super::players::{grid, random};

pub const NUM_ROWS: usize = 10;
pub const NUM_COLS: usize = 10;

pub const OFFSETS: [(i32, i32); 4] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1)
];

pub type ShootFn = fn(Player, ShotMap) -> Pos;
pub type PlaceFn = fn() -> BoatMap;


pub type BoatMap = [[Boat; NUM_ROWS]; NUM_COLS];
pub type ShotMap = [[Option<Shot>; NUM_ROWS]; NUM_COLS];

lazy_static!(
    pub static ref SHOOT_FNS: Vec<(String, ShootFn)> = vec![
        ("Random shoot".to_string(), random::shoot),
        ("Random shoot + random destroy".to_string(), random::find_and_random_destroy),
        ("Random shoot + destroy".to_string(), random::find_and_destroy),
        ("Grid shoot + destroy".to_string(), grid::find_and_destroy),
    ];

    pub static ref PLACE_FNS: Vec<(String, PlaceFn)> = vec![
        ("Random place".to_string(), random::place_boats_random),
        ("Sides place".to_string(), random::place_boats_sides),
    ];
);
