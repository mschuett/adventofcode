use crate::helper;
use itertools::Itertools;
use std::ops::Index;

type Point = (usize, usize);

#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up = 0,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Network {
    data: Vec<Vec<char>>,
}
impl From<&str> for Network {
    fn from(s: &str) -> Self {
        let mut data: Vec<Vec<char>> = Vec::new();
        for line in s.lines() {
            data.push(
                line.as_bytes()
                    .iter()
                    .map(|b| *b as char)
                    .collect::<Vec<char>>(),
            );
        }
        Network { data }
    }
}
impl Index<Point> for Network {
    type Output = char;
    fn index(&self, index: Point) -> &Self::Output {
        &self.data[index.1][index.0]
    }
}
impl Network {
    fn find_start(&self) -> Option<Point> {
        self.data[0]
            .iter()
            .position(|&x| x == '|')
            .map(|x| Some((x, 0)))
            .unwrap_or(None)
    }
    fn walk_to_node(&self, start: Point, direction: &Direction) -> Point {
        // walk in this direction, until we find a letter or a corner
        let (mut x, mut y) = start;
        match direction {
            Direction::Up => {
                y -= 1;
                while self[(x, y)] != '+' && !self[(x, y)].is_ascii_uppercase() {
                    y -= 1;
                }
            }
            Direction::Down => {
                y += 1;
                while self[(x, y)] != '+' && !self[(x, y)].is_ascii_uppercase() {
                    y += 1;
                }
            }
            Direction::Left => {
                x -= 1;
                while self[(x, y)] != '+' && !self[(x, y)].is_ascii_uppercase() {
                    x -= 1;
                }
            }
            Direction::Right => {
                x += 1;
                while self[(x, y)] != '+' && !self[(x, y)].is_ascii_uppercase() {
                    x += 1;
                }
            }
        }
        (x, y)
    }
    fn turn(&self, position: Point, old_direction: &Direction) -> Option<Direction> {
        // standing on a node, determine the next direction; or none at finish point
        let cur_char = self[position];
        if cur_char == 'Z' || cur_char == 'F' {
            None
        } else if cur_char.is_ascii_uppercase() {
            Some(old_direction.clone())
        } else if cur_char == '+' {
            let (x, y) = position;
            match *old_direction {
                Direction::Up | Direction::Down => {
                    if x > 1 && (self[(x - 1, y)] == '-' || self[(x - 1, y)].is_ascii_uppercase()) {
                        Some(Direction::Left)
                    } else if x < self.data[y].len() - 1
                        && (self[(x + 1, y)] == '-' || self[(x + 1, y)].is_ascii_uppercase())
                    {
                        Some(Direction::Right)
                    } else {
                        panic!("cannot determine direction")
                    }
                }
                Direction::Left | Direction::Right => {
                    if y > 1 && (self[(x, y - 1)] == '|' || self[(x, y - 1)].is_ascii_uppercase()) {
                        Some(Direction::Up)
                    } else if y < self.data.len() - 1
                        && (self[(x, y + 1)] == '|' || self[(x, y + 1)].is_ascii_uppercase())
                    {
                        Some(Direction::Down)
                    } else {
                        panic!("cannot determine direction")
                    }
                }
            }
        } else {
            panic!(
                "unexpected character in turn({:?}, {:?}): {}",
                position, old_direction, cur_char
            );
        }
    }
    fn walk_and_get_order(&self) -> Vec<char> {
        let mut result: Vec<char> = Vec::new();
        let mut cursor: Point = self.find_start().unwrap();
        let mut dir = Direction::Down;
        loop {
            cursor = self.walk_to_node(cursor, &dir);
            // println!("walked to {:?}: {}", cursor, self[cursor]);
            if (self[cursor]).is_ascii_uppercase() {
                result.push(self[cursor]);
            }
            match self.turn(cursor, &dir) {
                None => break,
                Some(new_dir) => dir = new_dir,
            }
        }
        result
    }
    fn walk_and_get_stepcount(&self) -> u32 {
        let mut step_sum = 1u32; // 1 because the start position counts as first step
        let mut cursor: Point = self.find_start().unwrap();
        let mut old_cursor: Point = cursor;
        let mut dir = Direction::Down;
        loop {
            cursor = self.walk_to_node(cursor, &dir);
            let steps = cursor.0.abs_diff(old_cursor.0) + cursor.1.abs_diff(old_cursor.1);
            old_cursor = cursor;
            step_sum += steps as u32;
            // println!("walked {} steps to {:?}: {}", steps, cursor, self[cursor]);
            match self.turn(cursor, &dir) {
                None => break,
                Some(new_dir) => dir = new_dir,
            }
        }
        step_sum
    }
}

fn solve_part1(input_text: String) -> String {
    let network = Network::from(input_text.as_str());
    network.walk_and_get_order().iter().join("")
}

fn solve_part2(input_text: String) -> String {
    let network = Network::from(input_text.as_str());
    network.walk_and_get_stepcount().to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 19)
        .expect("Could not fetch input");
    println!("Example: {}", solve_part1(example_text()));
    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("     |          \n     |  +--+    \n     A  |  C    \n F---|----E|--+ \n     |  |  |  D \n     +B-+  +--+ \n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "ABCDEF");
    }
}
