//! Provides the input struct.

use shape::coord::Coord;
use shape::polygon::Polygon;

/// The input for deserialization.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    /// The start of the path.
    pub start: Coord,

    /// The end of the path.
    pub end: Coord,

    /// Points that must be passed in order from start to end.
    #[serde(default = "Vec::new")]
    pub route: Vec<Coord>,

    /// The polygons that block the path.
    #[serde(rename = "polygon", default = "Vec::new")]
    pub polygons: Vec<Polygon>,
}
