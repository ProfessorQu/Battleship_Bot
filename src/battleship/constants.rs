pub const NUM_ROWS: usize = 10;
pub const NUM_COLS: usize = 10;

pub type Boat = usize;

pub const EMPTY: Boat = 0;
pub const DESTROYER: Boat = 1;
pub const SUBMARINE: Boat = 2;
pub const CRUISER: Boat = 3;
pub const BATTLESHIP: Boat = 4;
pub const CARRIER: Boat = 5;

pub const BOATS: [Boat; 5] = [DESTROYER, SUBMARINE, CRUISER, BATTLESHIP, CARRIER];

pub const MIN_SHOTS: usize = 2 + 3 + 3 + 4 + 5;

