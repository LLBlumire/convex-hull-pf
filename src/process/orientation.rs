use ::shape::coord::Coord;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Orientation {
    Colinear,
    Clockwise,
    Counterclockwise
}
impl Orientation {
    fn from_coords(p: Coord, q: Coord, r: Coord) -> Orientation {
        match (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y) {
            n if n > 0 => Orientation::Clockwise,
            n if n < 0 => Orientation::Counterclockwise,
            _ => Orientation::Colinear
        }
    }

    fn is_colinear(&self) -> bool {
        *self == Orientation::Colinear
    }
}