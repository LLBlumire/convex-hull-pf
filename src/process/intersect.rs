use ::shape::segment::Segment;
use ::process::orientation::Orientation;

fn intersect(p: Segment, q: Segment) -> bool {
    let o1 = Orientation::from_coords(p.a, q.a, p.b);
    let o2 = Orientation::from_coords(p.a, q.a, q.b);
    let o3 = Orientation::from_coords(p.b, q.b, p.a);
    let o4 = Orientation::from_coords(p.b, q.b, q.a);

    if (o1 != o2 && o3 != o4) {
        return true;
    }

    if (o1.is_colinear() && 
}