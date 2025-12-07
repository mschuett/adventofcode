use std::collections::BTreeSet;
use std::collections::BTreeMap;
use crate::helper;

// return splitter x positions by line, treat S as the first splitter on line 0
fn parse_input(input_text: &str) -> Vec<Vec<u32>> {
    let lines = input_text
        .lines()
        .collect::<Vec<&str>>();
    let mut splitter_x_positions: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut splitters_in_line: Vec<u32> = Vec::new();
        for (x, char) in line.chars().enumerate() {
            if char == '^' || char == 'S' {
                splitters_in_line.push(x as u32);
            }
        }
        splitter_x_positions.push(splitters_in_line);
    };
    splitter_x_positions
}

fn solve_part1(input_text: String) -> String {
    let splitters = parse_input(&input_text);
    let mut splitter_taken_count: u32 = 0;
    let mut beam_x_positions: BTreeSet<u32> = BTreeSet::new();
    // beam starts at start
    beam_x_positions.insert(*splitters[0].first().unwrap());
    for splitter_line in splitters[1..].iter() {
        let mut new_beam_x_pos: BTreeSet<u32> = BTreeSet::new();
        for x in beam_x_positions {
            if splitter_line.contains(&x) {
                new_beam_x_pos.insert(x-1);
                new_beam_x_pos.insert(x+1);
                splitter_taken_count += 1;
            } else {
                new_beam_x_pos.insert(x);
            }
        }
        beam_x_positions = new_beam_x_pos;
    }
    splitter_taken_count.to_string()
}


fn solve_part2_naive(input_text: String) -> String {
    // uses a std::Vec for the beams => way too many, way too slow, only works for example
    let splitters = parse_input(&input_text);
    let mut beams: u32 = 0;
    let mut beam_x_positions: Vec<u32> = Vec::new();
    beam_x_positions.push(*splitters[0].first().unwrap());
    for splitter_line in splitters[1..].iter() {
        if splitter_line.is_empty() { continue }
        let mut new_beam_x_pos: Vec<u32> = Vec::new();
        for x in beam_x_positions {
            if splitter_line.contains(&x) {
                new_beam_x_pos.push(x-1);
                new_beam_x_pos.push(x+1);
            } else {
                new_beam_x_pos.push(x);
            }
        }
        beam_x_positions = new_beam_x_pos;
        beams = beam_x_positions.len() as u32;
        println!("beams: {}", beams);
    }
    beams.to_string()
}

fn solve_part2(input_text: String) -> String {
    // use map to remember beam positions with beam counts
    let splitters = parse_input(&input_text);
    let mut beam_x_positions: BTreeMap<u32, u64> = BTreeMap::new();
    beam_x_positions.insert(*splitters[0].first().unwrap(), 1);

    for splitter_line in splitters[1..].iter() {
        if splitter_line.is_empty() { continue }
        let mut new_beam_x_pos: BTreeMap<u32, u64> = BTreeMap::new();

        for (x_pos, x_count) in beam_x_positions {
            if splitter_line.contains(&x_pos) {
                new_beam_x_pos.entry(x_pos-1)
                    .and_modify(|x| *x += x_count)
                    .or_insert(x_count);
                new_beam_x_pos.entry(x_pos+1)
                    .and_modify(|x| *x += x_count)
                    .or_insert(x_count);
            } else {
                // note the modify, because another splitter could have created the entry
                new_beam_x_pos.entry(x_pos)
                    .and_modify(|x| *x += x_count)
                    .or_insert(x_count);
            }
        }

        beam_x_positions = new_beam_x_pos;
        // println!("splitters: {:?}", splitter_line);
        // println!("beam map entries: {} {:?}",
        //          beam_x_positions.values().sum::<u32>(),
        //          beam_x_positions);
    }
    beam_x_positions.values().sum::<u64>().to_string()
}


pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2025, 7).expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}


fn example_text() -> String {
    String::from("\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "21");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "40");
    }
}
