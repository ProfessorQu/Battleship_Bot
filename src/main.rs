#![allow(dead_code)]

mod battleship;
use battleship::Game;
use battleship::Player;
use battleship::Random;

use crate::battleship::constants::NUM_ROWS;

fn main() {
    let _game = Game::new(
        Random::place_boats(),
        Random::place_boats()
    );

    let boats = Random::place_boats();
    println!("{}", "-".repeat(boats.len() * 3 + 2));
    for y in 0..NUM_ROWS {
        print!("|");
        for column in boats {
            let element = column[y];
            let value = if element == 0 {
                "-".to_string()
            } else {
                element.to_string()
            };

            print!(" {} ", value);
        }
        println!("|");
    }
    println!("{}", "-".repeat(boats.len() * 3 + 2));
}