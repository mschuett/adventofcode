use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use crate::helper;

type Steps = u32;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u8,
    y: u8,
}
struct Maze {
    walls: HashSet<Point>,
    locations: HashMap<Point, char>,
}
impl Maze {
    fn from_input(input: &str) -> Maze {
        let mut walls: HashSet<Point> = HashSet::new();
        let mut locations: HashMap<Point, char> = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    walls.insert(Point{x: x as u8, y: y as u8});
                } else if ch >= '0' && ch <= '9' {
                    locations.insert(Point{x: x as u8, y: y as u8}, ch);
                }
            }
        }
        Maze{walls, locations}
    }
    fn get_location_pos(&self, loc: char) -> Result<Point, String> {
        for (p, c) in self.locations.iter() {
            if *c == loc { return Ok(*p); }
        }
        Err("not found".to_string())
    }
    fn next_positions(&self, pos: &Point) -> Vec<Point> {
        // four directions
        vec![
            Point{x: pos.x-1, y: pos.y},
            Point{x: pos.x+1, y: pos.y},
            Point{x: pos.x, y: pos.y-1},
            Point{x: pos.x, y: pos.y+1},]
            .iter()
            .filter(|p| !self.walls.contains(p))
            .cloned().collect()
    }
    fn find_partial_routes_from(&self, start_char: char) -> HashMap<(char, char), Steps> {
        let start = self.get_location_pos(start_char).unwrap();
        let mut locs_to_visit: Vec<char> = self.locations
            .values()
            .filter_map(|c| if *c != start_char { Some(*c) } else { None })
            .collect();
        let mut visited: HashSet<Point> = HashSet::new();
        visited.insert(start);
        let mut queue: VecDeque<(Steps, Point)> = VecDeque::new();
        queue.push_back((0, start));
        let mut fewest_steps: HashMap<(char, char), Steps> = HashMap::new();

        let mut iterations: u32 = 0;
        loop {
            if queue.is_empty() { break; }
            let (steps, current_loc) = queue.pop_front().unwrap();

            iterations += 1;
            if iterations % (1u32 << 16) == 0 {
                println!(
                    "bfs from {}: it {}, cur steps {}, visited {}, queue len {}",
                    start_char,
                    iterations,
                    steps,
                    visited.len(),
                    queue.len()
                );
            }
            if self.locations.contains_key(&current_loc) {
                // we found a location
                let loc_char = self.locations[&current_loc];
                fewest_steps
                    .entry((start_char, loc_char))
                    .or_insert(steps);

                let todo_index = locs_to_visit.iter().position(|c| *c == loc_char);
                if todo_index.is_some() {
                    locs_to_visit.remove(todo_index.unwrap());
                }
                // done?
                if locs_to_visit.is_empty() { break; }
            }

            for next in self.next_positions(&current_loc) {
                if !visited.contains(&next) && !queue.contains(&(steps + 1, next)) {
                    queue.push_back((steps + 1, next));
                }
            }
        }
        fewest_steps
    }
    fn get_all_partial_routes(&self) -> HashMap<(char, char), Steps> {
        let mut mapmap: HashMap<(char, char), Steps> = HashMap::new();
        for loc in self.locations.values() {
            let partial_map = self.find_partial_routes_from(*loc);
            for (pair, steps) in partial_map.iter() {
                mapmap.insert(pair.clone(), *steps);
            }
        }
        mapmap
    }
    // the parameter is for part 2
    fn get_shortest_path(&self, with_return: bool) -> Steps {
        let map = self.get_all_partial_routes();

        // start with 0, so remove that element from permutations
        let locations: Vec<char> = self.locations
            .values()
            .filter(|&&c| c != '0')
            .cloned().collect();
        locations.iter()
            .permutations(locations.len())
            .map(|mut perm| {
                // re-add the zero
                perm.insert(0, &'0');
                if with_return {
                    perm.push(&'0');
                }
                perm
            })
            .map(|path| {
                // sum the partial steps for all pairs in path
                path
                    .windows(2)
                    .map(|pair|
                        map.get(&(*pair[0], *pair[1])).unwrap()
                    )
                    .sum::<u32>()
            })
            // get the shortest path
            .min().unwrap()
    }
}

fn solve_part1(input_text: String) -> String {
    let m = Maze::from_input(&input_text);
    m.get_shortest_path(false).to_string()
}

fn solve_part2(input_text: String) -> String {
    let m = Maze::from_input(&input_text);
    m.get_shortest_path(true).to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 24)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("\
###########
#0.1.....2#
#.#######.#
#4.......3#
###########
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "14");
    }
}
