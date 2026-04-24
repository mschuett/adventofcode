use std::collections::{HashMap,HashSet};
use crate::helper;

fn redistribute(banks: &mut Vec<u8>) -> u32 {
    let mut history: HashSet<Vec<u8>> = HashSet::new();
    let mut steps: u32 = 0;
    while !history.contains(banks) {
        // println!("step {}: {:?}", steps, banks);
        steps += 1;
        history.insert(banks.clone());

        let max_el = *banks.iter().max().unwrap();
        let max_index = banks.iter().position(|x| *x == max_el).unwrap();

        banks[max_index] = 0;
        for i in 1..=max_el as usize {
            let pos = (max_index + i) % banks.len();
            banks[pos] += 1;
        }
    }
    steps
}

fn solve_part1(input_text: String) -> String {
    let mut banks: Vec<u8> = input_text
        .trim()
        .split_whitespace()
        .map(|line| line.parse::<u8>().unwrap())
        .collect();
    redistribute(&mut banks).to_string()
}

fn redistribute_with_memory(banks: &mut Vec<u8>) -> u32 {
    let mut history: HashMap<Vec<u8>,u32> = HashMap::new();
    let mut steps: u32 = 0;
    while !history.contains_key(banks) {
        history.insert(banks.clone(), steps);
        steps += 1;

        let max_el = *banks.iter().max().unwrap();
        let max_index = banks.iter().position(|x| *x == max_el).unwrap();

        banks[max_index] = 0;
        for i in 1..=max_el as usize {
            let pos = (max_index + i) % banks.len();
            banks[pos] += 1;
        }
    }
    steps - history.get(banks).unwrap()
}

fn solve_part2(input_text: String) -> String {
    let mut banks: Vec<u8> = input_text
        .trim()
        .split_whitespace()
        .map(|line| line.parse::<u8>().unwrap())
        .collect();
    redistribute_with_memory(&mut banks).to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 6)
            .expect("Could not fetch input");
    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("0 2 7 0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "5");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "4");
    }
}
