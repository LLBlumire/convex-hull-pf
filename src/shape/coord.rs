//! Provides the Coord struct.

/// A coordinate in the cartesian plane.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coord {
    /// The x coordinate.
    pub x: i64,
    /// The y coordinate.
    pub y: i64,
}
