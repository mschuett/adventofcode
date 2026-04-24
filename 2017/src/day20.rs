use crate::helper;
use std::fmt;
use itertools::Itertools;

#[derive(Debug,Eq,PartialOrd,PartialEq,Clone,Copy)]
pub struct Coord3d {
    x: i32,
    y: i32,
    z: i32,
}
type Vector3d = Coord3d;
type Acc3d = Coord3d;

impl fmt::Display for Coord3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}
impl From<&str> for Coord3d {
    fn from(s: &str) -> Self {
        let str_numbers = s
            .strip_prefix("<").unwrap()
            .strip_suffix(">").unwrap()
            .splitn(3, ",")
            .collect::<Vec<&str>>();
        let (x, y, z) = str_numbers.iter()
            .map(|&s| s.trim().parse::<i32>().unwrap())
            .collect_tuple().unwrap();
        Coord3d {x, y, z}
    }
}
impl Coord3d {
    fn dist(&self, other: Coord3d) -> f32 {
        (
            ((self.x-other.x) as f32).powi(2) +
                ((self.y-other.y) as f32).powi(2) +
                ((self.z-other.z) as f32).powi(2)
        ).sqrt()
    }
    fn manhattan_dist(&self, other: Coord3d) -> u32 {
        self.x.abs_diff(other.x) +
        self.y.abs_diff(other.y) +
        self.z.abs_diff(other.z)
    }
    fn move_position(&self, other: Vector3d) -> Coord3d {
        Coord3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Vector3d {
    fn accelerate(&self, other: Acc3d) -> Vector3d {
        Coord3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
struct Particle {
    position: Coord3d,
    velocity: Vector3d,
    acceleration: Acc3d,
}
impl From<&str> for Particle {
    fn from(line: &str) -> Self {
        let parts = line.split(", ").collect::<Vec<&str>>();
        let position = Coord3d::from(parts[0].strip_prefix("p=").unwrap());
        let velocity = Coord3d::from(parts[1].strip_prefix("v=").unwrap());
        let acceleration = Coord3d::from(parts[2].strip_prefix("a=").unwrap());
        Particle {position, velocity, acceleration}
    }
}
impl Particle {
    fn time_tick(&mut self) {
        self.velocity = self.velocity.accelerate(self.acceleration);
        self.position = self.position.move_position(self.velocity);
    }
    fn forward(&mut self, t: u32) {
        for _ in 0..t {
            self.time_tick();
        }
    }
}

fn solve_part1(input_text: String) -> String {
    let mut particles: Vec<Particle> = input_text.lines().map(Particle::from).collect();

    let time = 10000;  // what is "long term"? :)
    particles.iter_mut()
        .for_each(|p| p.forward(time));

    let origin = Coord3d{x: 0, y: 0, z: 0};
    let (min_pos, min_val) = particles.iter()
        .map(|p| p.position.manhattan_dist(origin))
        .enumerate()
        .min_by_key(|item| item.1)
        .unwrap();
    min_pos.to_string()
}

fn solve_part2(input_text: String) -> String {
    let mut particles: Vec<Particle> = input_text.lines().map(Particle::from).collect();

    let max_time = 10000;
    for t in 0..max_time {
        particles.iter_mut().for_each(|p| p.time_tick());

        // find collisions
        // unoptimized delete-list to change list after all loops over it...
        let mut to_delete: Vec<usize> = Vec::new();
        for i in 0..particles.len() {
            for j in (i+1)..particles.len() {
                if i != j && particles[i].position == particles[j].position {
                    to_delete.push(i);
                    to_delete.push(j);
                }
            }
        }
        to_delete.sort();
        to_delete.reverse();
        for i in to_delete.iter().unique() {
            particles.remove(*i);
        }
    }
    particles.len().to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 20)
        .expect("Could not fetch input");

    println!("Example: {}", solve_part1(example_text()));
    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("\
p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "1");
    }
}
