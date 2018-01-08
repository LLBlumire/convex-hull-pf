use ::shape::coord::Coord;

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub shortest_hull: Vec<Coord>,
    pub longest_hull: Vec<Coord>,
}