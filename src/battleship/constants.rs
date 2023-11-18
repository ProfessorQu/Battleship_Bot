pub const NUM_ROWS: usize = 10;
pub const NUM_COLS: usize = 10;

pub const OFFSETS: [(i32, i32); 4] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1)
];

pub const MIN_SHOTS: usize = 2 + 3 + 3 + 4 + 5;
