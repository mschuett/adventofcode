use itertools::Itertools;
use crate::helper;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Ops {
    SwapPos{x: usize, y: usize},
    SwapLtr{x: char, y: char},
    RotLeft{x: usize},
    RotRight{x: usize},
    RotPos{x: char},
    ReversePos{x: usize, y: usize},
    MovePos{x: usize, y: usize},
}
impl Ops {
    fn from_string(line: &str) -> Ops {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        match words[0] {
            "swap" => {
                assert_eq!(words[3], "with");
                match words[1] {
                    "position" => {
                        let x: usize = words[2].parse().unwrap();
                        let y: usize = words[5].parse().unwrap();
                        Ops::SwapPos{x, y}
                    }
                    "letter" => {
                        let x: char = words[2].chars().nth(0).unwrap();
                        let y: char = words[5].chars().nth(0).unwrap();
                        Ops::SwapLtr{x, y}
                    }
                    _ => panic!("invalid swap op"),
                }
            }
            "rotate" => {
                match words[1] {
                    "left" => {
                        let x: usize = words[2].parse().unwrap();
                        assert!(words[3] == "steps" || words[3] == "step");
                        Ops::RotLeft{x}
                    }
                    "right" => {
                        let x: usize = words[2].parse().unwrap();
                        assert_eq!(words[3], "steps");
                        Ops::RotRight{x}
                    }
                    "based" => {
                        assert_eq!(words[2], "on");
                        assert_eq!(words[3], "position");
                        assert_eq!(words[4], "of");
                        assert_eq!(words[5], "letter");
                        let x: char = words[6].chars().nth(0).unwrap();
                        Ops::RotPos {x}
                    }
                    _ => panic!("invalid rotate op")
                }
            }
            "reverse" => {
                assert_eq!(words[1], "positions");
                assert_eq!(words[3], "through");
                let x: usize = words[2].parse().unwrap();
                let y: usize = words[4].parse().unwrap();
                Ops::ReversePos{x, y}
            }
            "move" => {
                assert_eq!(words[1], "position");
                assert_eq!(words[3], "to");
                assert_eq!(words[4], "position");
                let x: usize = words[2].parse().unwrap();
                let y: usize = words[5].parse().unwrap();
                Ops::MovePos{x, y}
            }
            _ => panic!("invalid op")
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Scrambler {
    text: Vec<char>,
}
impl Scrambler {
    fn new(text: &str) -> Scrambler {
        let text: Vec<char> = text
            .chars()
            .collect();
        Scrambler{text}
    }
    fn to_string(&self) -> String {
        self.text.iter().collect()
    }
    fn perform_op(&mut self, op: &Ops) {
        match op {
            Ops::SwapPos{x, y} => {
                self.text.swap(*x, *y);
            }
            Ops::SwapLtr{x, y} => {
                let x = self.text.iter().position(|a| a == x).unwrap();
                let y = self.text.iter().position(|a| a == y).unwrap();
                self.text.swap(x, y);
            }
            Ops::RotLeft{x} => {
                self.text.rotate_left(*x);
            }
            Ops::RotRight{x} => {
                self.text.rotate_right(*x);
            }
            Ops::RotPos{x} => {
                let len = self.text.len();
                let index = self.text.iter().position(|a| a == x).unwrap();
                let rotate = 1 + index + (if index >= 4 {1} else {0});
                self.text.rotate_right(rotate % len);
            }
            Ops::ReversePos{x, y} => {
                let mut new_text: Vec<char> = vec![];
                for i in 0..*x {
                    new_text.push(self.text[i]);
                }
                let part = &self.text.as_mut_slice()[*x..=*y];
                for c in part.into_iter().rev() {
                    new_text.push(*c);
                }
                for i in *y+1..self.text.len() {
                    new_text.push(self.text[i]);
                }
                self.text = new_text;
            }
            Ops::MovePos{x, y} => {
                let c = self.text.remove(*x);
                self.text.insert(*y, c);
            }
        }
    }
}
fn solve_part1(input_text: String, password: &str) -> String {
    let ops = input_text.lines()
        .map(Ops::from_string)
        .collect::<Vec<Ops>>();
    let mut scrambler = Scrambler::new(password);
    ops.iter().for_each(|op|
        scrambler.perform_op(op)
    );
    scrambler.to_string()
}

fn solve_part2(input_text: String, scrambled: &str) -> String {
    let ops = input_text.lines()
        .map(Ops::from_string)
        .collect::<Vec<Ops>>();

    // some ops are not reversible
    // so try every source permutation until we find the target
    let perms = scrambled.chars().permutations(scrambled.len());
    for p in perms {
        let mut scrambler = Scrambler::new(p.iter().join("").as_str());
        ops.iter().for_each(|op|
            scrambler.perform_op(op)
        );
        if scrambler.to_string() == scrambled {
            return p.iter().join("");
        }
    }
    "missing".to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 21)
            .expect("Could not fetch input");
    println!("Example: {}", solve_part1(example_text(), "abcde"));
    println!("Part 1: {}", solve_part1(input_text.clone(), "abcdefgh"));
    println!("Part 2: {}", solve_part2(input_text, "fbgdceah"));
}

fn example_text() -> String {
    String::from("\
swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 steps
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut s = Scrambler::new("abcde");
        assert_eq!(s.to_string(), "abcde");

        let o = Ops::from_string("swap position 4 with position 0");
        s.perform_op(&o);
        assert_eq!(s.to_string(), "ebcda");
        let o = Ops::from_string("swap letter d with letter b");
        s.perform_op(&o);
        assert_eq!(s.to_string(), "edcba");
        let o = Ops::from_string("reverse positions 0 through 4");
        s.perform_op(&o);
        assert_eq!(s.to_string(), "abcde");
        let o = Ops::from_string("rotate left 1 steps");
        s.perform_op(&o);
        assert_eq!(s.to_string(), "bcdea");
        let o = Ops::from_string("move position 1 to position 4");
        s.perform_op(&o);
        assert_eq!(s.to_string(), "bdeac");
        let o = Ops::from_string("move position 3 to position 0");
        s.perform_op(&o);
        assert_eq!(s.to_string(), "abdec");
        let o = Ops::from_string("rotate based on position of letter b");
        s.perform_op(&o);
        assert_eq!(s.to_string(), "ecabd");
        let o = Ops::from_string("rotate based on position of letter d");
        s.perform_op(&o);
        assert_eq!(s.to_string(), "decab");
    }
}
