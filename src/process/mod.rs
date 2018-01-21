//! Provides the process function, as well as housing the internals for computing convex hulls.

use io::input::Input;
use io::output::Output;
use shape::orientation::Orientation;
use shape::coord::Coord;
use shape::segment::Segment;
use std::collections::HashSet;
use shape::hull::Hull;
use std::hash::BuildHasher;

/// Processes the input into it's output by generating the convex hulls.
pub fn process(input: &Input) -> Output {
    let mut hulls = Vec::new();

    let mut path = input.route.clone();
    path.push(input.end);
    path.insert(0, input.start);

    let mut path = path.iter();

    let mut origin;
    let mut destination = path.next().unwrap(); // Guarunteed to have value
    'generate_all_hulls: loop {
        origin = destination;
        let destination_o = path.next();
        if destination_o.is_none() {
            break 'generate_all_hulls;
        }
        destination = destination_o.unwrap();

        let mut hull: HashSet<Segment> = HashSet::new();
        'generate_hull: loop {
            let mut polypoints = Segment::from_coords(*origin, *destination)
                .get_intersecting_polygon_coords(&input.polygons);

            for hull_segment in &hull {
                let union = polypoints
                    .union(&hull_segment.get_intersecting_polygon_coords(&input.polygons))
                    .cloned()
                    .collect();

                if polypoints == union {
                    break 'generate_hull;
                }

                polypoints = union;
            }

            polypoints.insert(*origin);
            polypoints.insert(*destination);

            hull = hull.union(&calculate_hull(&polypoints)).cloned().collect();
        }
        hulls.push(Hull::from_segment_set(hull.into_iter().collect()));
    }

    Output {
        input: input.clone(),
        hulls: hulls,
    }
}

/// Calculates the points that lie in the hull of a set of points.
pub fn calculate_hull<S: BuildHasher>(polypoints: &HashSet<Coord, S>) -> HashSet<Segment> {
    let mut hull = HashSet::new();

    if !quick_hull(polypoints, &mut hull) {
        panic!();
    }

    hull.iter().cloned().collect()
}

/// Calculates the quick hull of a set of points, outputting it into a buffer.
/// Returns true when computation is successful.
pub fn quick_hull<S1: BuildHasher, S2: BuildHasher>(
    input: &HashSet<Coord, S1>,
    hull: &mut HashSet<Segment, S2>,
) -> bool {
    if input.len() < 2 {
        return false;
    }

    let (leftest, rightest) = input.iter().fold(
        (None, None),
        |(mut leftest, mut rightest), &item| {
            if leftest.is_none() {
                leftest = Some(item);
            }
            if rightest.is_none() {
                rightest = Some(item);
            }
            if item.x < leftest.unwrap().x {
                leftest = Some(item);
            }
            if item.x > rightest.unwrap().x {
                rightest = Some(item);
            }
            (leftest, rightest)
        },
    );
    let leftest = leftest.unwrap();
    let rightest = rightest.unwrap();

    quick_hull_recurse(input, leftest, rightest, Orientation::Clockwise, hull);
    quick_hull_recurse(
        input,
        leftest,
        rightest,
        Orientation::Counterclockwise,
        hull,
    );

    true
}

/// The recursive call component of `quick_hull`.
fn quick_hull_recurse<S1: BuildHasher, S2: BuildHasher>(
    input: &HashSet<Coord, S1>,
    p1: Coord,
    p2: Coord,
    orientation: Orientation,
    hull: &mut HashSet<Segment, S2>,
) {
    let mut divider: Option<Coord> = None;
    let mut max_dist = 0;

    for &coord in input.iter() {
        let dist = Segment::from_coords(p1, p2).coord_distance(coord);
        if Orientation::from_coords(p1, p2, coord) == orientation && dist > max_dist {
            divider = Some(coord);
            max_dist = dist;
        }
    }

    if let Some(divider) = divider {
        quick_hull_recurse(
            input,
            divider,
            p1,
            Orientation::from_coords(divider, p1, p2).invert(),
            hull,
        );
        quick_hull_recurse(
            input,
            divider,
            p2,
            Orientation::from_coords(divider, p2, p1).invert(),
            hull,
        );
    } else {
        hull.insert(Segment::from_coords(p1, p2));
    }
}
