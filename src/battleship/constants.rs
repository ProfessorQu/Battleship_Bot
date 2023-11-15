pub const NUM_ROWS: usize = 10;
pub const NUM_COLS: usize = 10;

pub const DESTROYER: usize = 1;
pub const SUBMARINE: usize = 2;
pub const CRUISER: usize = 3;
pub const BATTLESHIP: usize = 4;
pub const CARRIER: usize = 5;

pub const BOATS: [usize; 5] = [DESTROYER, SUBMARINE, CRUISER, BATTLESHIP, CARRIER];
pub const LENGTHS: [usize; 6] = [0, 2, 3, 3, 4, 5];

pub const MIN_SHOTS: usize = 2 + 3 + 3 + 4 + 5;
