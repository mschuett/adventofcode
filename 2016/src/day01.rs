use std::collections::HashSet;
use crate::coord2d::{Coord2d, Point};
use crate::helper;

#[derive(Debug,PartialEq)]
pub enum Direction {
    N, E, S, W
}
#[derive(Debug,PartialEq)]
pub enum Turn {
    L,R
}
#[derive(Debug,PartialEq)]
pub struct Walker {
    pos: Coord2d,
    dir: Direction,
}

impl Walker {
    pub fn new() -> Walker {
        Walker{pos: Coord2d{x: 0, y: 0}, dir: Direction::N}
    }
    fn turn (&mut self, dir: Turn) {
        match self.dir {
            Direction::N => if dir == Turn::L { self.dir = Direction::W } else { self.dir = Direction::E },
            Direction::E => if dir == Turn::L { self.dir = Direction::N } else { self.dir = Direction::S },
            Direction::S => if dir == Turn::L { self.dir = Direction::E } else { self.dir = Direction::W },
            Direction::W => if dir == Turn::L { self.dir = Direction::S } else { self.dir = Direction::N },
        }
    }
    fn forward(&mut self, dist: i32) {
        match self.dir {
            Direction::N => { self.pos.y -= dist }
            Direction::S => { self.pos.y += dist }
            Direction::W => { self.pos.x -= dist }
            Direction::E => { self.pos.x += dist }
        }
    }
    pub fn step(&mut self, dir: Turn, dist: i32) {
        self.turn(dir);
        self.forward(dist);
    }
    // part 2; same as step, but return vec of all visited positions
    pub fn step_with_visited(&mut self, dir: Turn, dist: i32) -> Vec<Coord2d> {
        self.turn(dir);
        let mut visited: Vec<Coord2d> = vec![];
        for i in 0..dist {
            self.forward(1);
            visited.push(self.pos);
        }
        visited
    }
}

fn solve_part1(input_text: String) -> String {
    let mut w = Walker::new();

    let input1 = input_text
        .replace(" ", "")
        .replace("\n", "");
    let words = input1
        .split(",")
        .collect::<Vec<&str>>();
    for word in words {
        let dir: Turn;
        if word.as_bytes()[0] == 'L' as u8 {
            dir = Turn::L
        } else {
            dir = Turn::R
        }
        let dist = word[1..].parse::<i32>().unwrap();
        w.step(dir, dist);
    }
    w.pos.grid_dist(Coord2d {x: 0, y: 0}).to_string()
}

fn solve_part2(input_text: String) -> String {
    let mut w = Walker::new();

    let input1 = input_text
        .replace(" ", "")
        .replace("\n", "");
    let words = input1
        .split(",")
        .collect::<Vec<&str>>();
    let mut visited: HashSet<Coord2d> = HashSet::new();

    for word in words {
        let dir: Turn;
        if word.as_bytes()[0] == 'L' as u8 {
            dir = Turn::L
        } else {
            dir = Turn::R
        }
        let dist = word[1..].parse::<i32>().unwrap();
        let new_positions = w.step_with_visited(dir, dist);
        for pos in new_positions {
            if visited.contains(&pos) {
                return pos.grid_dist(Coord2d {x: 0, y: 0}).to_string()
            } else {
                visited.insert(pos);
            }
        }
    }
    w.pos.grid_dist(Coord2d {x: 0, y: 0}).to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 1).expect("Could not fetch input");
    let _ = example_text();

    print!(
        "Part 1: {}\nPart 2: {}\n",
        solve_part1(input_text.clone()),
        solve_part2(input_text)
    );
}

fn example_text() -> String {
    String::from(
        "R2, L3",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let result = solve_part1(example_text());
        assert_eq!(result, "5");
    }
    #[test]
    fn test_part1b() {
        let result = solve_part1(String::from("R2, R2, R2"));
        assert_eq!(result, "2");
    }
    #[test]
    fn test_part1c() {
        let result = solve_part1(String::from("R5, L5, R5, R3"));
        assert_eq!(result, "12");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(String::from("R8, R4, R4, R8"));
        assert_eq!(result, "4");
    }
}
