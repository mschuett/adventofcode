use memoize::memoize;
use md5::Digest;
use crate::helper;

fn has_triple_char(text: &str) -> Result<char, String> {
    for i in 0..text.len()-2 {
        if text.as_bytes()[i] == text.as_bytes()[i+1]
        && text.as_bytes()[i] == text.as_bytes()[i+2] {
            return Ok(char::from(text.as_bytes()[i]))
        }
    }
    Err("No triple char found".into())
}

fn has_quintuple_char(text: &str, c: char) -> bool {
    for i in 0..text.len()-4 {
        if text.as_bytes()[i] == c as u8
            && text.as_bytes()[i] == text.as_bytes()[i+1]
            && text.as_bytes()[i] == text.as_bytes()[i+2]
            && text.as_bytes()[i] == text.as_bytes()[i+3]
            && text.as_bytes()[i] == text.as_bytes()[i+4] {
            return true
        }
    }
    false
}

#[memoize]
fn indexed_digest(salt: String, index: u32, stretch: u32) -> String {
    let md5input = format!("{}{}", salt, index);
    let mut digest = md5::compute(md5input.as_bytes());
    let mut hex_digest = format!("{:x}", digest);

    for _ in 0..stretch {
        digest = md5::compute(hex_digest.as_bytes());
        hex_digest = format!("{:x}", digest);
    }
    hex_digest
}

fn find_next_key(salt: &str, start_index: u32, stretch: u32) -> u32 {
    let mut index = start_index;
    loop {
        let hex_digest = indexed_digest(salt.to_string(), index, stretch);
        match has_triple_char(&hex_digest) {
            Ok(c) => {
                for offset in 1..=1000 {
                    let next_digest = indexed_digest(salt.to_string(), index + offset, stretch);
                    if has_quintuple_char(&next_digest, c) { return index }
                }
            }
            Err(_) => {}  // loop and continue search
        }
        index += 1;
    }
}

fn solve_part1(input_text: String) -> String {
    let mut index: u32 = 0;
    let stretch= 0;
    for i in 0..64 {
        index = find_next_key(&input_text, index+1, stretch);
        // println!("{}: {}", i, index);
    }
    index.to_string()
}

fn solve_part2(input_text: String) -> String {
    let mut index: u32 = 0;
    let stretch= 2016;
    for i in 0..64 {
        index = find_next_key(&input_text, index+1, stretch);
        // println!("{}: {}", i, index);
    }
    index.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 14)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Example: {}", solve_part1(example_text()));
    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Example 2: {}", solve_part2(example_text()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from(
        "abc")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "22728");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "22551");
    }
}
