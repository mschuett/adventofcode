use std::fmt;
use std::cmp::Ordering;

#[derive(Debug,Eq,Ord,Clone,Copy)]
pub struct Coord3d {
    x: i32,
    y: i32,
    z: i32,
}
impl fmt::Display for Coord3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}
impl PartialEq for Coord3d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
            self.y == other.y &&
            self.z == other.z
    }
}
impl PartialOrd for Coord3d {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Coord3d {
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn from(input: &str) -> Coord3d {
        let v = input
            .splitn(3,',')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        Coord3d{x: v[0], y: v[1], z: v[2]}
    }
    pub fn dist(&self, other: Coord3d) -> f32 {
        (
            ((self.x-other.x) as f32).powi(2) +
            ((self.y-other.y) as f32).powi(2) +
            ((self.z-other.z) as f32).powi(2)
        ).sqrt()
    }
}
