use ::shape::coord::Coord;
use std::cmp::max;
use std::cmp::min;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Segment {
    pub a: Coord,
    pub b: Coord,
}
impl Segment {
    fn from_coords(a: Coord, b: Coord) {
        Segment {
            a,
            b
        }
    }

    fn contains_colinear_coord(&self, coord: Coord) -> bool {
        coord.x <= max(self.a.x, self.b.x) && q.x >= min(self.a.x, self.b.x) && coord.y <= max(self.a.y, self.b.y) && coord.y >= min(self.a.y, self.b.y)
    }
}