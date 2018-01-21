//! Provides the Polygon struct.

use shape::coord::Coord;
use shape::segment::Segment;

/// Represents a polygon.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Hash)]
pub struct Polygon {
    /// The set a points that make up the polygon, ordered counterclockwise.
    #[serde(rename = "point")]
    pub points: Vec<Coord>,
}

impl Polygon {
    /// Returns a vector of the segments that make up the polygon.
    pub fn segments(&self) -> Vec<Segment> {
        let cycleiter = self.points.iter().chain(self.points.iter().take(1));
        let cycleiter2 = self.points.iter().chain(self.points.iter().take(2)).skip(1);
        let edges = cycleiter
            .zip(cycleiter2)
            .map(|(&a, &b)| Segment::from_coords(a, b));
        edges.collect::<Vec<_>>()
    }
}
