use std::collections::HashSet;
use crate::helper;

type Point = (i32, i32);  // x, y

#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn turn_left(&mut self) {
        match self {
            Direction::Up => *self = Direction::Left,
            Direction::Left => *self = Direction::Down,
            Direction::Down => *self = Direction::Right,
            Direction::Right => *self = Direction::Up,
        }
    }
    fn turn_right(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        }
    }
    fn reverse(&mut self) {
        match self {
            Direction::Up => *self = Direction::Down,
            Direction::Right => *self = Direction::Left,
            Direction::Down => *self = Direction::Up,
            Direction::Left => *self = Direction::Right,
        }
    }
}

#[derive(Clone, Debug)]
struct Carrier {
    position: Point,
    direction: Direction,
}
impl Carrier {
    fn new() -> Self {
        Carrier{position: (0, 0), direction: Direction::Up}
    }
    fn step_forward(&mut self) {
        match self.direction {
            Direction::Up => self.position.1 -= 1,
            Direction::Down => self.position.1 += 1,
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
        }
    }
}

#[derive(Clone, Debug)]
struct Cluster {
    cur_infected: HashSet<Point>,
    burst_infections_counter: u32,
    carrier: Carrier,
    // part 2 extensions
    cur_weakened: HashSet<Point>,
    cur_flagged: HashSet<Point>,
}
impl From<String> for Cluster {
    fn from(input_text: String) -> Self {
        let mut cur_infected = HashSet::new();
        let burst_infections_counter = 0;
        let carrier = Carrier::new();

        let lines = input_text.trim().lines().collect::<Vec<_>>();
        let offset = lines.len() as i32 / 2;
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    let point = (x as i32 - offset, y as i32 - offset);
                    cur_infected.insert(point);
                }
            }
        }
        let cur_weakened = HashSet::new();
        let cur_flagged = HashSet::new();
        Cluster { cur_infected, burst_infections_counter, carrier, cur_weakened, cur_flagged }
    }
}
impl Cluster {
    fn simple_carrier_next(&mut self) {
        if self.cur_infected.contains(&self.carrier.position) {
            self.cur_infected.remove(&self.carrier.position);
            self.carrier.direction.turn_right();
        } else {
            self.cur_infected.insert(self.carrier.position);
            self.burst_infections_counter += 1;
            self.carrier.direction.turn_left();
        }
        self.carrier.step_forward();
    }
    fn evolved_carrier_next(&mut self) {
        if self.cur_infected.contains(&self.carrier.position) {
            self.cur_infected.remove(&self.carrier.position);
            self.cur_flagged.insert(self.carrier.position);
            self.carrier.direction.turn_right();
        } else if self.cur_flagged.contains(&self.carrier.position) {
            self.cur_flagged.remove(&self.carrier.position);
            self.carrier.direction.reverse();
        } else if self.cur_weakened.contains(&self.carrier.position) {
            self.cur_weakened.remove(&self.carrier.position);
            self.cur_infected.insert(self.carrier.position);
            self.burst_infections_counter += 1;
            // no turn
        } else /* clean node */ {
            self.cur_weakened.insert(self.carrier.position);
            self.carrier.direction.turn_left();
        }
        self.carrier.step_forward();
    }
}

fn solve_part1(input_text: String, iterations: u32) -> String {
    let mut cluster = Cluster::from(input_text);
    for _ in 0..iterations {
        cluster.simple_carrier_next();
    }
    cluster.burst_infections_counter.to_string()
}

fn solve_part2(input_text: String, iterations: u32) -> String {
    let mut cluster = Cluster::from(input_text);
    for _ in 0..iterations {
        cluster.evolved_carrier_next();
    }
    cluster.burst_infections_counter.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 22)
        .expect("Could not fetch input");

    println!("Part 1: {}", solve_part1(input_text.clone(), 10000));
    println!("Part 2: {}", solve_part2(input_text, 10000000));
}

fn example_text() -> String {
    String::from("\
..#
#..
...
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(example_text(), 70), "41");
        assert_eq!(solve_part1(example_text(), 10000), "5587");
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(example_text(), 100), "26");
        assert_eq!(solve_part2(example_text(), 10000000), "2511944");
    }
}
