use nalgebra::geometry::Point2;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

impl Coord {
    pub fn as_point2(&self) -> Point2<f64> {
        Point2::new(self.x, self.y)
    }
}
