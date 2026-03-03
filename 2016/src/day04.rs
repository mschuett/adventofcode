use crate::helper;
use std::collections::HashMap;

fn check_top5chars(input: &str, given: &str) -> bool {
    let mut counts: HashMap<char, u16> = HashMap::new();
    for char in input.chars() {
        if char == '-' { continue }
        *counts.entry(char).or_insert(0u16) += 1;
    }
    let given_chars = given.chars().collect::<Vec<char>>();
    let max_count = *counts.values().max().unwrap();
    // check a few checksum constraints
    if *counts.entry(given_chars[0]).or_default() != max_count { return false }
    if *counts.entry(given_chars[0]).or_default() < *counts.entry(given_chars[1]).or_default() { return false }
    if *counts.entry(given_chars[1]).or_default() < *counts.entry(given_chars[2]).or_default() { return false }
    if *counts.entry(given_chars[2]).or_default() < *counts.entry(given_chars[3]).or_default() { return false }
    if *counts.entry(given_chars[3]).or_default() < *counts.entry(given_chars[4]).or_default() { return false }
    true
}

fn calc_checksum(input: &str) -> String {
    let mut counts: HashMap<char, u16> = HashMap::new();
    for char in input.chars() {
        if char == '-' { continue }
        *counts.entry(char).or_insert(0u16) += 1;
    }

    let mut tops: Vec::<(char, u16)> = counts
        .into_iter()
        .collect::<Vec<(char, u16)>>();
    tops
        .sort_by(|(a_char, _), (b_char, _)| a_char.cmp(b_char));
    tops
        .sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));
    tops
        .iter()
        .take(5)
        .map(|(char, _)| char)
        .collect::<String>()
}

fn solve_part1(input_text: String) -> String {
    let mut sum: u32 = 0;
    for line in input_text.lines() {
        let (name, idcheck) = line
            .rsplit_once("-").unwrap()
            .into();
        let (id_str, check) = idcheck
            .strip_suffix("]").unwrap()
            .split_once("[").unwrap()
            .into();
        let id = id_str.parse::<u32>().unwrap();
        // println!("{} {}", line, calc_checksum(name));
        if calc_checksum(name) == check {
            sum += id
        }
    }
    sum.to_string()
}

fn decrypt_name(name: &str, id: u32) -> String {
    let mut output: Vec<char> = vec![];
    for char in name.chars() {
        if char == '-' {
            output.push(' ');
            continue
        }
        let c = char as u8 - 'a' as u8;
        let new_c = (((c as u32 + id) % 26) as u8) + 'a' as u8;
        output.push(new_c as char);
    }
    output.into_iter().collect::<String>()
}

fn solve_part2(input_text: String) -> String {
    for line in input_text.lines() {
        let (name, idcheck) = line
            .rsplit_once("-").unwrap()
            .into();
        let (id_str, check) = idcheck
            .strip_suffix("]").unwrap()
            .split_once("[").unwrap()
            .into();
        let id = id_str.parse::<u32>().unwrap();
        if calc_checksum(name) == check {
            // println!("{} {}", decrypt_name(name, id), id);
            let clear_name = decrypt_name(name, id);
            if clear_name.contains("northpole") {
                return id.to_string();
            }
        }
    }
    "not found".to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 4)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let result = solve_part1(String::from("aaaaa-bbb-z-y-x-123[abxyz]"));
        assert_eq!(result, "123");
    }
    fn test_part1b() {
        let result = solve_part1(String::from("a-b-c-d-e-f-g-h-987[abcde]"));
        assert_eq!(result, "987");
    }
    fn test_part1c() {
        let result = solve_part1(String::from("not-a-real-room-404[oarel]"));
        assert_eq!(result, "404");
    }
    fn test_part1d() {
        let result = solve_part1(String::from("totally-real-room-200[decoy]"));
        assert_eq!(result, "0");
    }
    fn test_part2() {
        let result = decrypt_name("qzmt-zixmtkozy-ivhz", 343);
        assert_eq!(result, "very encrypted name");
    }
}
