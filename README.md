# Battleship_Bot

This is an implementation of Battleship. It implements a few bots.

## Quickstart

```rust
use battleship_bot::{Battleship, place, shoot};

let mut game = Battleship::new(
    place::sides,
    place::spread,

    shoot::random_and_destroy,
    shoot::grid_and_destroy
);

let recording = game.play_and_record_game();

println!("{} won", recording.winner);
```

## Shooting

First of all we have the random shooting, completely random in `shoot::random`.
Then, there's a bit more sophisticated with `shoot::random_and_random_destroy`, which shoots randomly at the outer ends. To round of the random shooting methods we have `shoot::random_and_destroy`, which uses the previous shots we've had to determine how the ship is facing and then using that.

After all that we have `shoot::grid_and_destroy`, which uses a different algorithm for finding but then destroys in the same way `shoot::random_and_destroy` does. It shoots in a grid pattern and destroys ships it comes across.

Finally, last but certainly not least we have `shoot::heatmap_and_destroy`. It generates a heatmap to determine what the greatest changes are that a ship will be placed on a tile, then shoots the highest tile and a random one if there are multiple highest.

### Shooting your own shots

If you want to create your own implementation of a shoot function, it can be passed into `Battleship::new` exactly the same way as the others. As long as it accepts a `Pos`, which is the last position shot at and a `ShotMap` which is a 2D array containing all the `Shot`s taken for this player.

## Placing

For placing we'll start with `place::random`, which, much like its `shoot` counterpart places ships completely randomly.

Second we have `place::sides` which places the boats at a random position along the sides.

After that we have `place::spread` which places each boat in a certain quadrant, they are divided from top-left to bottom-right. The Destroyer (2 length) has the first quadrant, the Submarine (3 length) has the second and so on until the Carrier (5 length) which is the fifth boat. The Carrier is therefore able to be placed anywhere.

Finally we have `place::cluster` which clusters all boats in the center of the board together.

### Placing your own boats

If you want to create your own implementation of a place function, it doesn't have to accept variables, all it has to do is return a `BoatMap`. `BoatMap` is a type alias for a 2D array with `Boat`s.
