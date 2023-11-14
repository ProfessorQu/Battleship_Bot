#![allow(dead_code)]

mod battleship;
use battleship::Game;
use battleship::Player;
use battleship::Random;

fn main() {
    let game = Game::new(
        Random::place_boats(),
        Random::place_boats()
    );

    println!("--------------------------------");
    for row in Random::place_boats() {
        print!("|");
        for element in row {
            if element != 0 {
                print!(" {element:?} ")
            }
            else {
                print!("   ")
            }
        }

        println!("|");
    }
    println!("--------------------------------");
}