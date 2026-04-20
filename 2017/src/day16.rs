use crate::helper;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum DanceMove {
    Spin(u8),
    Exchange(u8, u8),
    Partner(char, char),
}
impl TryFrom<&str> for DanceMove {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.chars().nth(0).unwrap() {
            's' => {
                let num = value[1..].parse::<u8>().unwrap();
                Ok(DanceMove::Spin(num))
            },
            'x' => {
                let (left, right) = value[1..].split_once('/').unwrap();
                let leftnum = left.parse::<u8>().unwrap();
                let rightnum = right.parse::<u8>().unwrap();
                Ok(DanceMove::Exchange(leftnum, rightnum))
            },
            'p' => {
                let (left, right) = value[1..].split_once('/').unwrap();
                let leftchar = left.chars().nth(0).unwrap();
                let rightchar = right.chars().nth(0).unwrap();
                Ok(DanceMove::Partner(leftchar, rightchar))
            },
            _ => Err(format!("Invalid dance move: {}", value)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Programs {
    list: Vec<char>
}
impl Programs {
    fn new(size: u8) -> Self {
        let list = Vec::from_iter(
            (0..size).map(|i| (b'a' + i) as char));
        Programs { list }
    }
    fn do_move(&mut self, mv: DanceMove) {
        match mv {
            DanceMove::Spin(num) => {
                self.list.rotate_right(num as usize);
            }
            DanceMove::Exchange(left, right) => {
                self.list.swap(left as usize, right as usize);
            }
            DanceMove::Partner(left, right) => {
                let i_left = self.list.iter().position(|&x| x == left).unwrap();
                let i_right = self.list.iter().position(|&x| x == right).unwrap();
                self.list.swap(i_left, i_right);
            }
        }
    }
    fn do_all_moves(&mut self, moves: Vec<DanceMove>) {
        for mv in moves {
            self.do_move(mv);
        }
    }
    fn as_string(&self) -> String {
        self.list.iter().collect()
    }
}

fn parse_input(input: &str) -> Vec<DanceMove> {
    input.trim()
        .split(",")
        .map(|mov| DanceMove::try_from(mov.trim()).unwrap())
        .collect::<Vec<DanceMove>>()
}

fn solve_part1(input_text: String, size: u8) -> String {
    let moves = parse_input(&input_text);
    let mut p = Programs::new(size);
    p.do_all_moves(moves);
    p.as_string()
}

fn solve_part2(input_text: String, size: u8) -> String {
    let moves = parse_input(&input_text);
    let mut p = Programs::new(size);

    // determine cycle length and save all results
    let mut states: Vec<String> = Vec::new();
    loop {
        states.push(p.as_string());
        p.do_all_moves(moves.clone());
        if p.as_string() == states[0] {
            break;
        }
    }
    // now return the correct entry
    let cycle_index = 1_000_000_000usize % states.len();
    states[cycle_index].clone()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 16)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Example: {}", solve_part1(example_text(), 5));
    println!("Part 1: {}", solve_part1(input_text.clone(), 16));
    println!("Part 2: {}", solve_part2(input_text, 16));
}

fn example_text() -> String {
    String::from("s1, x3/4, pe/b")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text(), 5);
        assert_eq!(result, "baedc");
    }
}
