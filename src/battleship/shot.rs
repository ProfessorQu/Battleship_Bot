use std::fmt::Debug;

use crate::Boat;

/// Stores the data for a shot
/// 
/// Can either be a `Miss` or a `Hit`.
/// If it's a `Hit` it stores the [`Boat`] it hit
#[derive(Clone, Copy)]
pub enum Shot {
    Hit(Boat),
    Miss
}

impl Debug for Shot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hit(boat) => write!(f, "{:?}", boat),
            Self::Miss => write!(f, "M"),
        }
    }
}
