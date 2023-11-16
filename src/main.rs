#![allow(dead_code)]

mod battleship;

use battleship::Battleship;
use battleship::Random;

fn main() {
    let mut battleship = Battleship::new(
        Random::place_boats_random,
        Random::place_boats_random,

        Random::shoot_random,
        Random::shoot_random,
    );

    let (p1_won, p2_won) = battleship.play_games(10_000);

    println!("p1: {}; p2: {}", p1_won, p2_won);
}