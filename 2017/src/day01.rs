use itertools::Itertools;
use crate::helper;

fn count_pairs(text: &str) -> u32 {
    let mut sum = text.chars()
        .tuple_windows::<(_, _)>()
        .filter_map(|(a, b)|
            if a == b {
                Some(a.to_digit(10).unwrap())
            } else { None })
        .sum::<u32>();
    if text.chars().nth_back(0) == text.chars().nth(0) {
        sum += text.chars()
            .nth(0).unwrap()
            .to_digit(10).unwrap();
    }
    sum
}

fn count_opposing_pairs(text: &str) -> u32 {
    let size = text.len();
    let half = size / 2;
    text.chars()
        .enumerate()
        .filter_map(|(i, a)|
            if a == text.chars().nth((i + half) % size).unwrap() {
                Some(a.to_digit(10).unwrap())
            } else { None })
        .sum::<u32>()
}


fn solve_part1(input_text: String) -> String {
    count_pairs(input_text.trim()).to_string()
}

fn solve_part2(input_text: String) -> String {
    count_opposing_pairs(input_text.trim()).to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 1)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from(
        "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(count_pairs("1122"), 3);
        assert_eq!(count_pairs("1111"), 4);
        assert_eq!(count_pairs("1234"), 0);
        assert_eq!(count_pairs("91212129"), 9);
    }

    #[test]
    fn test_part2() {
        assert_eq!(count_opposing_pairs("1212"), 6);
        assert_eq!(count_opposing_pairs("1221"), 0);
        assert_eq!(count_opposing_pairs("123425"), 4);
        assert_eq!(count_opposing_pairs("123123"), 12);
        assert_eq!(count_opposing_pairs("12131415"), 4);
    }
}
