#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x, y
        }
    }
}

#[macro_export]
macro_rules! pos {
    ($t:expr) => {
        Pos::new($t.0, $t.1)
    };
    ($x:expr, $y:expr) => {
        Pos::new($x, $y)
    };
}