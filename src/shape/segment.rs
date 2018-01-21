//! Provides the Segment struct.

use shape::coord::Coord;
use std::cmp::max;
use std::cmp::min;
use shape::orientation::Orientation;
use std::collections::HashSet;
use shape::polygon::Polygon;
use std::cmp::Ordering;

/// Represents a line segment AB.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Segment {
    /// The left start of the line segment.
    pub a: Coord,

    /// The right end of the line segment.
    pub b: Coord,
}
impl Segment {
    /// Constructs a line segment from it's coordinates AB.
    pub fn from_coords(a: Coord, b: Coord) -> Segment {
        let (a, b) = match (a.x.cmp(&b.x), a.y.cmp(&b.y)) {
            (Ordering::Less, _) | (Ordering::Equal, Ordering::Less) => (a, b),
            _ => (b, a),
        };
        Segment { a, b }
    }

    /// Checks if a point lies on self.
    pub fn contains_colinear_coord(&self, coord: Coord) -> bool {
        if Orientation::from_coords(self.a, self.b, coord).is_colinear() {
            (coord.x <= max(self.a.x, self.b.x) && coord.x >= min(self.a.x, self.b.x)
                && coord.y <= max(self.a.y, self.b.y)
                && coord.y >= min(self.a.y, self.b.y))
        } else {
            false
        }
    }

    /// Checks if self intersects another line segment.
    pub fn intersects(&self, other: &Segment) -> bool {
        if self.a == other.a || self.a == other.b || self.b == other.a || self.b == other.b {
            return false;
        }

        let o1 = Orientation::from_coords(self.a, self.b, other.a);
        let o2 = Orientation::from_coords(self.a, self.b, other.b);
        let o3 = Orientation::from_coords(other.a, other.b, self.a);
        let o4 = Orientation::from_coords(other.a, other.b, self.b);

        if o1 != o2 && o3 != o4 {
            return true;
        }

        if o1.is_colinear() && Segment::from_coords(self.a, self.b).contains_colinear_coord(other.a)
        {
            return true;
        }

        if o2.is_colinear() && Segment::from_coords(self.a, self.b).contains_colinear_coord(other.b)
        {
            return true;
        }

        if o3.is_colinear()
            && Segment::from_coords(other.a, other.b).contains_colinear_coord(self.a)
        {
            return true;
        }

        if o4.is_colinear()
            && Segment::from_coords(other.a, other.b).contains_colinear_coord(self.b)
        {
            return true;
        }

        false
    }

    /// Returns a value proportional to the distance between the line (extended
    /// of the segment) and the points.
    pub fn coord_distance(&self, other: Coord) -> i64 {
        ((other.y - self.a.y) * (self.b.x - self.a.x)
            - (self.b.y - self.a.y) * (other.x - self.a.x))
            .abs()
    }

    /// Finds the polygons that intersect with a segment.
    pub fn get_intersecting_polygons(&self, polygons: &[Polygon]) -> HashSet<Polygon> {
        let mut intersecting_polygons = HashSet::new();
        'p: for polygon in polygons.iter() {
            for polygon_segment in polygon.segments() {
                if polygon_segment.intersects(self) {
                    intersecting_polygons.insert(polygon.clone());
                    continue 'p;
                }
            }
        }
        intersecting_polygons
    }

    /// Finds the coordinates of polygons that intersect the segment.
    pub fn get_intersecting_polygon_coords(&self, polygons: &[Polygon]) -> HashSet<Coord> {
        let mut intersecting_polygons = HashSet::new();
        'p: for polygon in polygons.iter() {
            for polygon_segment in polygon.segments() {
                if polygon_segment.intersects(self) {
                    for coord in &polygon.points {
                        intersecting_polygons.insert(coord.clone());
                    }
                    continue 'p;
                }
            }
        }
        intersecting_polygons
    }
}
