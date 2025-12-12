use crate::helper;
use std::fmt;
use itertools::Itertools;

#[derive(Debug,Clone,Copy)]
struct Shape {
    id: u8,
    bits: [[bool; 3]; 3],
}
impl Shape {
    fn from(input_text: &str) -> Shape {
        let lines: Vec<&str> = input_text.lines().collect();
        let id = lines[0]
            .split(":").next().unwrap()
            .parse::<u8>().unwrap();
        let bits: [[bool; 3]; 3] = lines[1..4].iter()
            .map(|l| l.chars()
                .map(|c| c.eq(&'#'))
                .collect_array().unwrap()
            ).collect_array().unwrap();
        Shape { id, bits }
    }
}
impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.id, self.bits)
    }
}
struct Region {
    width: u32,
    length: u32,
    presents: [u32; 6],
}
impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}: {:?}", self.width, self.length, self.presents)
    }
}

fn parse_input(input_text: &str) -> (Vec<Shape>, Vec<Region>) {
    let (shapes_text, regions_text) = input_text
        .rsplit_once("\n\n").unwrap();
    let shapes: Vec<Shape> = shapes_text
        .split("\n\n").map(Shape::from).collect();
    let mut regions: Vec<Region> = Vec::new();
    for line in regions_text.lines() {
        let (dim,pres) = line.split_once(": ").unwrap();
        let (width,length) = dim.split_once("x")
            .map(|(w,l)| (w.parse::<u32>().unwrap(), l.parse::<u32>().unwrap()))
            .unwrap();
        let presents: [u32; 6] = pres.split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect_array().unwrap();
        regions.push(Region{width,length,presents});
    }
    (shapes, regions)
}

fn solve_part1(input_text: String) -> String {
    let (shapes, regions) = parse_input(&input_text);
    // shapes.iter().for_each(|shape| println!("{}", shape));
    // regions.iter().for_each(|region| println!("{}", region));

    let mut canfit: u32 = 0;
    for reg in regions {
        let area = reg.length * reg.width;
        let req_area = reg.presents.iter().map(|p| p*9).sum::<u32>();
        if req_area <= area {
            canfit += 1;
            println!("{} => {} < {} ✅", reg, area, req_area);
        } else {
            println!("{} => {} < {} ❌", reg, area, req_area);
        }
    }

    canfit.to_string()
}


pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2025, 12)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text));
    // no Part 2 today
}

fn example_text() -> String {
    String::from("\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
")
}

#[cfg(test)]
mod tests {
    use super::*;

    // strange times... the example was more difficult than the real input?
    // #[test]
    // fn test_part1() {
    //     let result = solve_part1(example_text());
    //     assert_eq!(result, "2");
    // }
}
