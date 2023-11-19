use lazy_static::lazy_static;

use super::{boat::Boat, game::Shot, Player, Pos};
use super::players::{place, shoot};

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
        ("Randshot".to_string(), shoot::random_shoot),
        ("Randshot+randdestr".to_string(), shoot::random_find_and_random_destroy),
        ("Randshot+destr".to_string(), shoot::random_find_and_destroy),
        ("Gridshot+destr".to_string(), shoot::grid_find_and_destroy),
        ("Heatshot+destr".to_string(), shoot::heatmap_find_and_destroy)
    ];

    pub static ref PLACE_FNS: Vec<(String, PlaceFn)> = vec![
        ("Randplace".to_string(), place::place_boats_random),
        ("Sideplace".to_string(), place::place_boats_sides),
        ("Spreadplace".to_string(), place::place_boats_spread),
        ("Clusterplace".to_string(), place::place_boats_cluster)
    ];
);
