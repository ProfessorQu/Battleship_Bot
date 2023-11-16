#![allow(dead_code)]

mod battleship;

use battleship::Battleship;
use battleship::PlayerTrait;
use battleship::Random;

fn main() {
    let mut battleship = Battleship::new(
        Random::place_boats,
        Random::place_boats,

        Random::shoot,
        Random::shoot,
    );

    let (p1_won, p2_won) = battleship.play_games(10_000);

    println!("p1: {}; p2: {}", p1_won, p2_won);
}