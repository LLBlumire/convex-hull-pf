use io::common::coord::Coord;
use io::input::polygon::Polygon;

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub start: Coord,
    pub end: Coord,
    #[serde(default = "Vec::new")]
    pub route: Vec<Coord>,
    #[serde(rename = "polygon", default = "Vec::new")]
    pub polygons: Vec<Polygon>,
}
