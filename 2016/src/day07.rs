use crate::helper;

fn get_bracketed_strings(input: &str) -> (Vec<&str>, Vec<&str>){
    let mut clear_strings: Vec<&str> = vec![];
    let mut bracket_strings: Vec<&str> = vec![];
    let mut start: usize = 0;
    let mut i: usize = 0;
    loop {
        while i < input.len() && input.as_bytes()[i] != '[' as u8 {
            i += 1
        }
        clear_strings.push(&input[start..i]);
        if i == input.len() {
            return (clear_strings, bracket_strings)
        } else if input.as_bytes()[i] == '[' as u8 {
            i += 1;
            start = i;
            while input.as_bytes()[i] != ']' as u8 {
                i += 1
            }
            bracket_strings.push(&input[start..i]);
            i += 1;
            start = i;
        }
    }
}

fn contains_abba(word: &str) -> bool {
    let bytes = word.as_bytes();
    for i in 0..bytes.len()-3 {
        if bytes[i] != bytes[i+1]
            && bytes[i] == bytes[i+3]
            && bytes[i+1] == bytes[i+2] {
            return true;
        }
    }
    false
}

fn supports_tls(input: &str) -> bool {
    let (clear_strings, bracket_strings) = get_bracketed_strings(input);
    for word in bracket_strings {
        if contains_abba(word) { return false; }
    }
    for word in clear_strings {
        if contains_abba(word) { return true; }
    }
    false
}

fn solve_part1(input_text: String) -> String {
    let mut counter = 0;
    for line in input_text.lines() {
        if supports_tls(line) { counter += 1; }
    }
    counter.to_string()
}

fn get_all_abas(word: &str) -> Vec<&str> {
    let mut result: Vec<&str> = vec![];
    let bytes = word.as_bytes();
    for i in 0..bytes.len()-2 {
        if bytes[i] != bytes[i+1]
            && bytes[i] == bytes[i+2] {
                let aba = &word[i..i+3];
                result.push(aba);
        }
    }
    result
}

fn aba_to_bab(aba: &str) -> String {
    let bytes = aba.as_bytes();
    [bytes[1] as char,
     bytes[0] as char,
     bytes[1] as char].iter().collect::<String>()
}

fn supports_ssl(input: &str) -> bool {
    let (clear_strings, bracket_strings) = get_bracketed_strings(input);

    let all_abas = clear_strings
        .iter()
        .map(|word| get_all_abas(word))
        .flatten()
        .collect::<Vec<&str>>();

    for word in bracket_strings {
        for aba in all_abas.iter() {
            let bab = aba_to_bab(aba);
            if word.contains(bab.as_str()) {
                return true
            }
        }
    }
    false
}

fn solve_part2(input_text: String) -> String {
    let mut counter = 0;
    for line in input_text.lines() {
        if supports_ssl(line) { counter += 1; }
    }
    counter.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 7)
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
    fn test_bracketed_strings() {
        let res = get_bracketed_strings("abba[mnop]qrst");
        assert_eq!(res.0, vec!["abba", "qrst"]);
        assert_eq!(res.1, vec!["mnop"]);
    }

    #[test]
    fn test_part1a() {
        assert!(supports_tls("abba[mnop]qrst"));
    }

    #[test]
    fn test_part1b() {
        assert!(!supports_tls("abcd[bddb]xyyx"));
    }

    #[test]
    fn test_part1c() {
        assert!(!supports_tls("aaaa[qwer]tyui"));
    }

    #[test]
    fn test_part1d() {
        assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn test_part2a() {
        let result = get_all_abas("zazbz");
        assert_eq!(result, vec!["zaz", "zbz"]);
    }

    #[test]
    fn test_part2b() {
        let result = aba_to_bab("aba");
        assert_eq!(result, "bab");
        assert!("xxbabxx".contains("bab"));
        assert!("xxbabxx".contains(result.as_str()));
        assert!(!"xxbxaxbxx".contains(result.as_str()));
        assert!(!"xxbabxx".contains("aba"));
    }

    #[test]
    fn test_part2c() {
        assert!(supports_ssl("aba[bab]xyz"));
        assert!(!supports_ssl("xyx[xyx]xyx"));
        assert!(supports_ssl("aaa[kek]eke"));
        assert!(supports_ssl("zazbz[bzb]cdb"));
    }
}
