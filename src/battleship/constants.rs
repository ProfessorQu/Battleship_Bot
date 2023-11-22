use lazy_static::lazy_static;

use crate::battleship::{boat::Boat, game::Shot};
use crate::player::players::{place, shoot};

use super::game::Player;
use super::position::Pos;

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
        ("Randshot".to_string(), shoot::random),
        ("Randshot+randdestr".to_string(), shoot::random_and_random_destroy),
        ("Randshot+destr".to_string(), shoot::random_and_destroy),
        ("Gridshot+destr".to_string(), shoot::grid_and_destroy),
        ("Heatshot+destr".to_string(), shoot::heatmap_and_destroy)
    ];

    pub static ref PLACE_FNS: Vec<(String, PlaceFn)> = vec![
        ("Randplace".to_string(), place::random),
        ("Sideplace".to_string(), place::sides),
        ("Spreadplace".to_string(), place::spread),
        ("Clusterplace".to_string(), place::cluster)
    ];

    pub static ref FNS: Vec<((String, PlaceFn), (String, ShootFn))> = PLACE_FNS
        .iter().zip(SHOOT_FNS.iter()).map(|item| (item.0.clone(), item.1.clone())).collect();
);
