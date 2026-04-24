use itertools::Itertools;
use crate::helper;

fn row_checksum(row: &Vec<u16>) -> u16 {
    let min = row.iter().min().unwrap();
    let max = row.iter().max().unwrap();
    max - min
}

fn solve_part1(input_text: String) -> String {
    let mut checksum: u16 = 0;
    for line in input_text.lines() {
        let numbers = line
            .split_whitespace()
            .map(|word| word.parse::<u16>().unwrap())
            .collect_vec();
        checksum += row_checksum(&numbers);

    }
    checksum.to_string()
}

fn division_checksum(numbers: &Vec<u16>) -> u16 {
    for i in 0..numbers.len() {
        for j in i+1..numbers.len() {
            let smaller = numbers[i].min(numbers[j]);
            let larger = numbers[i].max(numbers[j]);
            if larger % smaller == 0 {
                return larger / smaller;
            }
        }
    }
    panic!("did not find divisible numbers in row");
}

fn solve_part2(input_text: String) -> String {
    let mut checksum: u16 = 0;
    for line in input_text.lines() {
        let numbers = line
            .split_whitespace()
            .map(|word| word.parse::<u16>().unwrap())
            .collect_vec();
        checksum += division_checksum(&numbers);

    }
    checksum.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 2)
            .expect("Could not fetch input");

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("\
5 1 9 5
7 5 3
2 4 6 8
")}

fn example_text2() -> String {
    String::from("\
5 9 2 8
9 4 7 3
3 8 6 5
")}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "18");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text2());
        assert_eq!(result, "9");
    }
}
