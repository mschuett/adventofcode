use std::collections::HashMap;
use itertools::Itertools;
use crate::helper;

struct Pattern {
    size: u8,
    bits: Vec<String>,
    replace: Vec<String>,
}
impl From<&str> for Pattern {
    fn from(line: &str) -> Self {
        let (from, to) = line.split_once(" => ").unwrap();
        let bits: Vec<String> = from.split("/").map(String::from).collect();
        let replace: Vec<String> = to.split("/").map(String::from).collect();
        let size = bits[0].len() as u8;
        assert!(size == 2 || size == 3);
        Pattern {size, bits, replace}
    }
}
impl std::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pattern")
            .field("size", &self.size)
            .field("rule", &format_args!("{} => {}",
                                         &self.bits.join("/"),
                                         &self.replace.join("/")))
            .finish()
    }
}
impl Pattern {
    fn rotate(bits: Vec<String>) -> Vec<String> {
        let size = bits.len();
        let mut result: Vec<String> = Vec::new();
        for i in 0..size {
            let mut line: Vec<u8> = Vec::new();
            for row in &bits {
                line.push(row.as_bytes()[size-i-1]);
            }
            result.push(line.iter().map(|b| *b as char).join(""));
        }
        result
    }
    fn flip(grid: Vec<String>) -> Vec<String> {
        let mut result = grid.clone();
        result.swap(0, grid.len()-1);
        result
    }
    fn apply(&self, grid: &Vec<String>) -> Option<Vec<String>> {
        let mut work_bits = self.bits.clone();
        for _rotations in 0..4 {
            // no need to flip 2x2 subgrids, so only 3x3 get that 2nd try
            if work_bits == *grid
                || (grid.len() == 3 && Self::flip(work_bits.clone()) == *grid) {
                // Never rotate or flip the output pattern, though.
                return Some(self.replace.clone());
            }
            work_bits = Self::rotate(work_bits);
        }
        None
    }
}

struct PatternSet {
    list: Vec<Pattern>,
    memo_cache: HashMap<Vec<String>, Vec<String>>,
}
impl From<&str> for PatternSet {
    fn from(input_text: &str) -> Self {
        let list = input_text.lines().map(Pattern::from).collect();
        let memo_cache = HashMap::new();
        PatternSet{list, memo_cache }
    }
}
impl PatternSet {
    fn apply(&mut self, grid: &Vec<String>) -> Vec<String> {
        if self.memo_cache.contains_key(grid) {
            return self.memo_cache.get(grid).unwrap().clone();
        }
        for pattern in &self.list {
            if grid.len() != pattern.bits.len() {
                continue;
            }
            match pattern.apply(grid) {
                None => continue,
                Some(replacement) => {
                    self.memo_cache.insert(grid.clone(), replacement.clone());
                    return replacement;
                }
            }
        }
        unreachable!("no pattern matched in apply() for {}", grid.join("/"));
    }
}


struct Grid {
    data: Vec<String>,
    patterns: PatternSet,
}
impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Grid")
            .field("data", &format_args!("{}", &self.data.join("\n             ")))
            .field("patterns", &format_args!("{} patterns", self.patterns.list.len()))
            .finish()
    }
}
impl Grid {
    fn new(patterns: PatternSet) -> Self {
        let data = vec![String::from(".#."), String::from("..#"), String::from("###")];
        Grid { data, patterns }
    }

    fn split_into_subgrids(grid: Vec<String>) -> Vec<Vec<String>> {
        // a lot of string copying :-(
        // but I don't know how to construct "2d-slices", to use parts of the existing memory
        let size = grid[0].len();
        let factor: usize;
        if size.is_multiple_of(2) {
            factor = 2;
        } else if size.is_multiple_of(3) {
            factor = 3;
        } else {
            panic!("invalid grid size")
        }

        let mut sub_grids: Vec<Vec<String>> = Vec::new();
        let subgrid_width = size / factor;
        for y in 0..subgrid_width {
            for x in 0..subgrid_width {
                let subgrid: Vec<String> = grid[factor*y..factor*(y+1)].to_vec()
                    .iter().map(|line| line[factor*x..factor*(x+1)].to_string())
                    .collect();
                sub_grids.push(subgrid);
            }
        }
        sub_grids
    }
    fn merge_from_subgrids(subgrids: Vec<Vec<String>>) -> Vec<String> {
        let grid_size = subgrids.len().isqrt();  // how many subgrid tiles per edge
        let tile_size = subgrids[0].len();  // how many pixels in every tile edge
        // let total_size = grid_size * tile_size;  // how many pixels in merged total
        let mut merged_grid: Vec<String> = Vec::with_capacity(grid_size);

        for sg_y in 0..grid_size {
            for tile_y in 0..tile_size {
                let line: String = subgrids[(sg_y*grid_size)..(sg_y+1)*grid_size]
                    .iter().map(|s| s[tile_y].clone())
                    .join("");
                merged_grid.push(line);
            }
        }
        merged_grid
    }

    fn next(&mut self) {
        let sub_grids: Vec<Vec<String>> = Self::split_into_subgrids(self.data.clone());
        let transformed_subgrids: Vec<Vec<String>> = sub_grids.iter()
            .map(|sg| self.patterns.apply(sg))
            .collect();
        self.data = Self::merge_from_subgrids(transformed_subgrids);
    }

    fn count_on_pixels(&self) -> usize {
        self.data.iter()
            .map(|line| line.as_bytes().iter().filter(|&b| *b == b'#').count())
            .sum()
    }
}


fn solve_parts(input_text: String, iterations: u16) -> String {
    let patterns = PatternSet::from(input_text.as_str());
    let mut grid = Grid::new(patterns);
    for i in 0..iterations {
        // println!("iteration #{} with\n{:?}", i, grid);
        grid.next();
    }
    grid.count_on_pixels().to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 21)
        .expect("Could not fetch input");

    println!("Example: {}", solve_parts(example_text(), 2));
    println!("Part 1: {}", solve_parts(input_text.clone(), 5));
    println!("Part 2: {}", solve_parts(input_text, 18));
}

fn example_text() -> String {
    String::from("\
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate1() {
        let data = vec![String::from(".#."), String::from("..#"), String::from("###")];
        assert_eq!(data, Pattern::rotate(Pattern::rotate(Pattern::rotate(Pattern::rotate(data.clone())))));
    }

    #[test]
    fn test_rotate2() {
        let data0 = vec![String::from("#."), String::from("..")];
        let data1 = vec![String::from(".."), String::from("#.")];
        let data2 = vec![String::from(".."), String::from(".#")];
        let data3 = vec![String::from(".#"), String::from("..")];
        assert_eq!(data1, Pattern::rotate(data0.clone()));
        assert_eq!(data2, Pattern::rotate(data1.clone()));
        assert_eq!(data3, Pattern::rotate(data2.clone()));
        assert_eq!(data0, Pattern::rotate(data3.clone()));
    }

    #[test]
    fn test_part1() {
        let result = solve_parts(example_text(), 2);
        assert_eq!(result, "12");
    }
}
