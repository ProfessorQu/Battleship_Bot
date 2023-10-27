#![allow(dead_code)]

use raylib::prelude::*;

mod battleship;
use battleship::Game;

fn main() {
    let (mut rl, thread) = init()
        .size(640, 640)
        .title("Hello World")
        .build();

    let game = Game::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        // game.draw(&mut d);

    }
}
