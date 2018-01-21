//! Provides the Orientation enum.
use shape::coord::Coord;

/// Represents the orientation of three points.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Orientation {
    /// Three points are colinear.
    Colinear,

    /// Three points are in clockwise orientation.
    Clockwise,

    /// Three points are in counterclockwise orientation.
    Counterclockwise,
}
impl Orientation {
    /// Computes an orientation from coordinates.
    pub fn from_coords(p: Coord, q: Coord, r: Coord) -> Orientation {
        match (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y) {
            n if n < 0 => Orientation::Clockwise,
            n if n > 0 => Orientation::Counterclockwise,
            _ => Orientation::Colinear,
        }
    }

    /// Returns true if Orientation is Colinear.
    pub fn is_colinear(self) -> bool {
        self == Orientation::Colinear
    }

    /// Returns the opposite orientation, Clockwise becomes Counterclockwise, Counterclockwise
    /// becomes Clockwise, and Colinear stays the same.
    pub fn invert(self) -> Self {
        match self {
            Orientation::Colinear => Orientation::Colinear,
            Orientation::Clockwise => Orientation::Counterclockwise,
            Orientation::Counterclockwise => Orientation::Clockwise,
        }
    }
}
