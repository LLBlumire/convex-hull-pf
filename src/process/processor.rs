use process::processor_output::ProcessorOutput;
use io::input::input::Input;
use nalgebra::geometry::Point2;
use ncollide::shape::Polyline2;
use std::sync::Arc;

pub struct Processor {
    start: Point2<f64>,
    end: Point2<f64>,
    route: Vec<Point2<f64>>,
    polygons: Vec<Polyline2<f64>>,
}
impl Processor {
    pub fn new(input: &Input) -> Processor {
        Processor {
            start: input.start.as_point2(),
            end: input.end.as_point2(),
            route: input.route.iter().map(|point| point.as_point2()).collect(),
            polygons: input
                .polygons
                .iter()
                .map(|polygon| {
                    let points = polygon
                        .points
                        .iter()
                        .map(|point| point.as_point2())
                        .collect::<Vec<_>>();
                    let index = (0..points.len())
                        .map(|index| Point2::new(index, index + 1 % points.len()))
                        .collect();
                    Polyline2::new(Arc::new(points), Arc::new(index), None, None)
                })
                .collect(),
        }
    }

    pub fn process(self) -> ProcessorOutput {
        unimplemented!()
    }
}
