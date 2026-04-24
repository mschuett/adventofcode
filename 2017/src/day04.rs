use crate::helper;

fn is_valid_phrase1(phrase: &str) -> bool {
    let words = phrase.split_whitespace().collect::<Vec<&str>>();
    for i in 0..words.len() {
        for j in i+1..words.len() {
            if words[i] == words[j] {
                return false
            }
        }
    }
    true
}

fn solve_part1(input_text: String) -> String {
    input_text
        .lines()
        .filter(|line| is_valid_phrase1(line))
        .count()
        .to_string()
}

fn is_anagram(a: &str, b: &str) -> bool {
    // quick check / micro-optimization
    if a.len() != b.len() || b.find(a.chars().nth(0).unwrap()).is_none() {
        return false
    }
    let mut buf: Vec<char> = b.chars().collect();
    for c in a.chars() {
        let pos = buf.iter().position(|&x| x == c);
        if pos.is_none() {
            return false
        }
        buf.remove(pos.unwrap());
    }
    buf.is_empty()  // anagram if all chars in b are "used" and matched
}

fn is_valid_phrase2(phrase: &str) -> bool {
    let words = phrase.split_whitespace().collect::<Vec<&str>>();
    for i in 0..words.len() {
        for j in i+1..words.len() {
            if is_anagram(words[i], words[j]) {
                return false
            }
        }
    }
    true
}

fn solve_part2(input_text: String) -> String {
    input_text
        .lines()
        .filter(|line| is_valid_phrase2(line))
        .count()
        .to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 4)
            .expect("Could not fetch input");
    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(is_valid_phrase1("aa bb cc dd ee"), true);
        assert_eq!(is_valid_phrase1("aa bb cc dd aa"), false);
        assert_eq!(is_valid_phrase1("aa bb cc dd aaa"), true);
    }

    #[test]
    fn test_part2() {
        assert_eq!(is_valid_phrase2("abcde fghij"), true);
        assert_eq!(is_valid_phrase2("abcde xyz ecdab"), false);
        assert_eq!(is_valid_phrase2("a ab abc abd abf abj"), true);
        assert_eq!(is_valid_phrase2("iiii oiii ooii oooi oooo"), true);
        assert_eq!(is_valid_phrase2("oiii ioii iioi iiio"), false);
    }
}
