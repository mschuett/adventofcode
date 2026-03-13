use crate::helper;
use std::collections::VecDeque;

// returns chars, repeat
fn read_marker(input: &str) -> (usize, usize) {
    assert_eq!(input.chars().nth(0).unwrap(), '(');
    assert_eq!(input.chars().last().unwrap(), ')');
    let (a, b) = input
        .strip_prefix('(').unwrap()
        .strip_suffix(')').unwrap()
        .split_once('x').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn decompress_line(input: &str) -> String {
    let mut output: Vec<char> = vec![];
    let mut i = 0;
    while i < input.len() {
        let cur_char = input.chars().nth(i).unwrap();
        if cur_char != '(' {
            output.push(cur_char);
            i += 1;
            continue;
        }
        assert_eq!(cur_char, '(');
        let offset = input[i..].find(')').unwrap() + 1;
        let (chars, repeat) = read_marker(&input[i .. i + offset]);
        i = i + offset;
        let rep_chars = input[i..(i+chars as usize)].chars().collect::<Vec<_>>();
        for _ in 0..repeat {
            for c in rep_chars.iter() {
                output.push(*c);
            }
        }
        i = i + chars as usize;
    }
    output.iter().collect()
}

fn solve_part1(input_text: String) -> String {
    let mut decompressed_length = 0;
    for line in input_text.lines() {
        let decompressed = decompress_line(line);
        decompressed_length += decompressed.len();
    }
    decompressed_length.to_string()
}

// for part 2: every line is a sequence of either marker or Characters
#[derive(Debug,PartialEq,Clone)]
struct Entity {
    plaintext: String,
    chars: usize,
    repeat: usize,
}
#[derive(Debug,PartialEq,Clone)]
struct Text {
    text: Vec<Entity>,
}

impl Text {
    fn new(input: &str) -> Self {
        let mut output: Vec<Entity> = vec![];
        let mut i = 0;
        while i < input.len() {
            let cur_char = input.chars().nth(i).unwrap();
            if cur_char == '(' {
                let offset = input[i..].find(')').unwrap() + 1;
                let plaintext = input[i .. i + offset].to_string();
                let (chars, repeat) = read_marker(&*plaintext);
                output.push(Entity {
                    plaintext,
                    chars,
                    repeat
                });
                i = i + offset;
            } else {
                let offset = input[i..].find('(');
                match offset {
                    Some(offset) => {
                        output.push(Entity {
                            plaintext: input[i..i + offset].to_string(),
                            chars: 0,
                            repeat: 0 });
                        i = i + offset;
                    }
                    None => {
                        output.push(Entity {
                            plaintext: input[i..].to_string(),
                            chars: 0,
                            repeat: 0 });
                        i = input.len()  // same as break
                    }
                }
            }
        }
        Text { text: output }
    }

    // wrapper to prepare VecDeque for dec_len
    fn len(&self) -> usize {
        let deque = VecDeque::from(self.text.clone());
        Text::dec_len(deque)
    }

    // recursively count decompressed chars of VecDeque
    fn dec_len(mut text: VecDeque<Entity>) -> usize {
        if text.is_empty() {
            return 0;
        }
        let head = text.pop_front().unwrap();
        if head.chars == 0 {
            return head.plaintext.len() + Text::dec_len(text);
        }

        // find all following entities for range in head.chars
        let mut missing_chars = head.chars;
        let mut to_repeat: VecDeque<Entity> = VecDeque::new();

        while missing_chars > 0 {
            let e = text.pop_front().unwrap();
            if e.plaintext.len() <= missing_chars {
                missing_chars -= e.plaintext.len();
                to_repeat.push_back(e);
            } else {
                // split the plaintext Entity to get only the first few chars
                if e.chars != 0 {
                    unimplemented!("cannot split marker!, bad input?")
                }
                let new_repeat = Entity {
                        plaintext: e.plaintext[0..missing_chars].to_string(),
                        chars: 0,
                        repeat: 0,
                    };
                let new_remaining = Entity {
                        plaintext: e.plaintext[missing_chars..].to_string(),
                        chars: 0,
                        repeat: 0,
                    };
                missing_chars -= new_repeat.plaintext.len();
                to_repeat.push_back(new_repeat);
                text.push_front(new_remaining);
            }
        }
        head.repeat * Text::dec_len(to_repeat) + Text::dec_len(text)
    }
}

fn solve_part2(input_text: String) -> String {
    let mut length = 0;
    for line in input_text.lines() {
        length += Text::new(line).len();
    }
    length.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 9)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from(
        "ADVENT
A(1x5)BC
(3x3)XYZ
A(2x2)BCD(2x2)EFG
(6x1)(1x3)A
X(8x2)(3x3)ABCY")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        assert_eq!(read_marker("(8x2)"), (8, 2));
        assert_eq!(read_marker("(14x2)"), (14, 2));
        assert_eq!(read_marker("(123x456)"), (123, 456));
    }

    #[test]
    fn test_part1b() {
        assert_eq!(decompress_line("ADVENT"), "ADVENT");
        assert_eq!(decompress_line("A(1x5)BC"), "ABBBBBC");
        assert_eq!(decompress_line("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress_line("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(decompress_line("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(decompress_line("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, (6+7+9+11+6+18).to_string());
    }

    #[test]
    fn test_part2() {
        let mut result: usize;

        result = Text::new("ADVENT").len();
        assert_eq!(result, "ADVENT".len());

        result = Text::new("A(1x5)BC").len();
        assert_eq!(result, "ABBBBBC".len());

        result = Text::new("(3x3)XYZ").len();
        assert_eq!(result, "XYZXYZXYZ".len());

        result = Text::new("X(8x2)(3x3)ABCY").len();
        assert_eq!(result, "XABCABCABCABCABCABCY".len());

        result = Text::new("(27x12)(20x12)(13x14)(7x10)(1x12)A").len();
        assert_eq!(result, 241920);

        result = Text::new("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN").len();
        assert_eq!(result, 445);
    }

    #[test]
    fn test_part2a() {
        assert_eq!(Text::new("ADVENT"), Text { text: vec![
            Entity { plaintext: "ADVENT".to_string(), chars: 0, repeat: 0 }
        ] });
        assert_eq!(Text::new("(12x5)"), Text { text: vec![
            Entity { plaintext: "(12x5)".to_string(), chars: 12, repeat: 5 }
        ] });
        assert_eq!(Text::new("A(1x5)BC"), Text { text: vec![
            Entity { plaintext: "A".to_string(), chars: 0, repeat: 0 },
            Entity { plaintext: "(1x5)".to_string(), chars: 1, repeat: 5 },
            Entity { plaintext: "BC".to_string(), chars: 0, repeat: 0 },
        ] });
    }
}
