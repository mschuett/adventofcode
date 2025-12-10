use std::fmt;
use std::cmp::Ordering;

pub type Point = Coord2d;

#[derive(Debug,Eq,Ord,Clone,Copy)]
pub struct Coord2d {
    pub x: i32,
    pub y: i32,
}
impl fmt::Display for Coord2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
impl PartialEq for Coord2d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y
    }
}
impl PartialOrd for Coord2d {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Coord2d {
    pub fn from(input: &str) -> Coord2d {
        let v = input
            .splitn(2,',')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        Coord2d{x: v[0], y: v[1]}
    }
    pub fn dist(&self, other: Coord2d) -> f32 {
        (
            ((self.x-other.x) as f32).powi(2) +
            ((self.y-other.y) as f32).powi(2)
        ).sqrt()
    }
    pub fn area(&self, other: Coord2d) -> i64 {
        ((self.x-other.x).abs()+1) as i64 * ((self.y-other.y).abs()+1) as i64
    }
}
