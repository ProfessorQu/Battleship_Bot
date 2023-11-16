#![allow(dead_code)]

mod battleship;

use battleship::Game;
use battleship::PlayerTrait;
use battleship::Player;
use battleship::Random;

fn main() {
    let mut p1_won = 0;
    let mut p2_won = 0;

    let mut game = Game::new(
        Random::place_boats,
        Random::place_boats,

        Random::shoot,
        Random::shoot
    );

    for _ in 0..1000 {
        let won = game.play();

        if matches!(won, Player::P1) {
            p1_won += 1;
        } else if matches!(won, Player::P2) {
            p2_won += 1;
        }
    }

    println!("p1: {}; p2: {}", p1_won, p2_won);
}