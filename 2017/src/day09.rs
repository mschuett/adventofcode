use crate::helper;

fn parse_groups(s: &str) -> (u32, u32, u32) {
    let bytes = s.as_bytes();
    let mut current_score: u32 = 1;
    let mut group_count: u32 = 0;
    let mut group_score: u32 = 0;
    let mut garbage_chars: u32 = 0;
    let mut i: usize = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'{' => {
                group_score += current_score;
                current_score += 1;
                i += 1;
            }
            b'}' => {
                group_count += 1;
                current_score -= 1;
                i += 1;
            }
            b'!' => {
                i += 2;
            }
            b'<' => {
                let mut j = i+1;
                loop {
                    match bytes[j] {
                        b'>' => {
                            i = j + 1;
                            break;
                        },
                        b'!' => {
                            j += 2;
                        },
                        _ => {
                            j += 1;
                            garbage_chars += 1;
                        }
                    }
                }
            }
            _ => {
                i += 1;
            }
        }
    }
    (group_count, group_score, garbage_chars)
}

fn solve_part1(input_text: String) -> String {
    let (_, score, _) = parse_groups(input_text.as_str());
    score.to_string()
}

fn solve_part2(input_text: String) -> String {
    let (_, _, garbage) = parse_groups(input_text.as_str());
    garbage.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 9)
            .expect("Could not fetch input");
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
    fn test_parse_groups() {
        assert_eq!(parse_groups("{}").0, 1);
        assert_eq!(parse_groups("{{{}}}").0, 3);
        assert_eq!(parse_groups("{{}{}}").0, 3);
        assert_eq!(parse_groups("{{{},{},{{}}}}").0, 6);
        assert_eq!(parse_groups("{<{},{},{{}}>}").0, 1);
        assert_eq!(parse_groups("{<a>,<a>,<a>,<a>}").0, 1);
        assert_eq!(parse_groups("{{<a>},{<a>},{<a>},{<a>}}").0, 5);
        assert_eq!(parse_groups("{{<!>},{<!>},{<!>},{<a>}}").0, 2);
    }

    #[test]
    fn test_score() {
        assert_eq!(parse_groups("{}").1, 1);
        assert_eq!(parse_groups("{{{}}}").1, 6);
        assert_eq!(parse_groups("{{}{}}").1, 5);
        assert_eq!(parse_groups("{{{},{},{{}}}}").1, 16);
        assert_eq!(parse_groups("{<a>,<a>,<a>,<a>}").1, 1);
        assert_eq!(parse_groups("{{<ab>},{<ab>},{<ab>},{<ab>}}").1, 9);
        assert_eq!(parse_groups("{{<!!>},{<!!>},{<!!>},{<!!>}}").1, 9);
        assert_eq!(parse_groups("{{<a!>},{<a!>},{<a!>},{<ab>}}").1, 3);
    }

    // part 2
    #[test]
    fn test_garbage() {
        assert_eq!(parse_groups("<>").2, 0);
        assert_eq!(parse_groups("<random characters>").2, 17);
        assert_eq!(parse_groups("<<<<>").2, 3);
        assert_eq!(parse_groups("<{!>}>").2, 2);
        assert_eq!(parse_groups("<!!>").2, 0);
        assert_eq!(parse_groups("<!!!>>").2, 0);
        assert_eq!(parse_groups("<{o\"i!a,<{i<a>").2, 10);
    }
}
