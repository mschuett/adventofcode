use std::collections::HashMap;
use crate::helper;

fn count_chars_into_maps(input_text: String) -> Vec<HashMap<char, u16>> {
    let mut chars_by_position: Vec<HashMap<char, u16>> = vec![];

    // init maps per position
    let line_length = input_text.find('\n').unwrap();
    for _ in 0..line_length {
        chars_by_position.push(HashMap::with_capacity(26));
    }
    for line in input_text.lines() {
        for (i, c) in line.chars().enumerate() {
            chars_by_position[i].entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
    }
    chars_by_position
}

fn get_most_common_chars(count_maps: &Vec<HashMap<char, u16>>) -> Vec<char> {
    let mut result: Vec<char> = vec![];
    for pos in 0..count_maps.len() {
        let (key_with_max_count, _) = count_maps[pos]
            .iter()
            .max_by_key(| kv | kv.1)
            .unwrap();
        result.push(*key_with_max_count)
    }
    result
}

fn get_least_common_chars(count_maps: &Vec<HashMap<char, u16>>) -> Vec<char> {
    let mut result: Vec<char> = vec![];
    for pos in 0..count_maps.len() {
        let (key_with_max_count, _) = count_maps[pos]
            .iter()
            .min_by_key(| kv | kv.1)
            .unwrap();
        result.push(*key_with_max_count)
    }
    result
}

fn solve_part1(input_text: String) -> String {
    let chars_by_position = count_chars_into_maps(input_text);
    let result = get_most_common_chars(&chars_by_position);
    result.iter().collect()
}

fn solve_part2(input_text: String) -> String {
    let chars_by_position = count_chars_into_maps(input_text);
    let result = get_least_common_chars(&chars_by_position);
    result.iter().collect()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 6)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from(
        "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "easter");
    }

    // #[test]
    // fn test_part2() {
    //     let result = solve_part2(example_text());
    //     assert_eq!(result, "0");
    // }
}
