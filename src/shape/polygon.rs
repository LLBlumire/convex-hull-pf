use ::shape::coord::Coord;

#[derive(Serialize, Deserialize, Debug)]
pub struct Polygon {
    #[serde(rename = "point")]
    pub points: Vec<Coord>,
}
