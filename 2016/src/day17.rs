use std::collections::VecDeque;
use crate::helper;

const SIZE: usize = 4;
type Point2d = (usize, usize);

enum Directions {
    Up = 0,
    Down,
    Left,
    Right,
}
impl Directions {
    fn as_char(&self) -> char {
        match self {
            Directions::Up => 'U',
            Directions::Down => 'D',
            Directions::Left => 'L',
            Directions::Right => 'R',
        }
    }
    fn step(&self, pos: Point2d) -> Point2d {
        match self {
            Directions::Up => (pos.0, pos.1 - 1),
            Directions::Down => (pos.0, pos.1 + 1),
            Directions::Left => (pos.0 - 1, pos.1),
            Directions::Right => (pos.0 + 1, pos.1),
        }
    }
}

fn digest(passcode: &str, path: &String) -> String {
    let md5input = format!("{}{}", passcode, path);
    let digest = md5::compute(md5input.as_bytes());
    let hex_digest = format!("{:x}", digest);
    hex_digest[..4].to_string()
}

fn is_open(c: char) -> bool {
    c >= 'b' && c <= 'f'
}

fn doors_open(hash: &str) -> [bool; 4] {
    core::array::from_fn(
        |i| is_open(char::from(hash.as_bytes()[i]))
    )
}

fn next_dirs(pos: Point2d, passcode: &str, path: &String) -> Vec<Directions> {
    let mut result: Vec<Directions> = vec![];
    let (x, y) = pos;
    let open = doors_open(&*digest(passcode, &path));
    // possible directions, considering doors and maze boundaries
    if open[Directions::Up as usize]    && y > 0      {result.push(Directions::Up)}
    if open[Directions::Down as usize]  && y < SIZE-1 {result.push(Directions::Down)}
    if open[Directions::Left as usize]  && x > 0      {result.push(Directions::Left)}
    if open[Directions::Right as usize] && x < SIZE-1 {result.push(Directions::Right)}
    result
}

fn bfs_search_shortest(passcode: String) -> String {
    const START: Point2d = (0, 0);
    const TARGET: Point2d = (3, 3);
    let mut queue: VecDeque<(Point2d, String)> = VecDeque::new();
    queue.push_back((START, "".to_string()));

    let mut iterations: u32 = 0;
    loop {
        let (pos, path) = queue
            .pop_front()
            .expect(format!("error, empty search queue after iteration {}", iterations).as_str());
        iterations += 1;
        if iterations % 16 == 0 {
            println!(
                "it {}, queue len {}, cur pos {:?}, cur steps {}",
                iterations,
                queue.len(),
                pos,
                path
            );
        }
        if pos == TARGET {
            return path;
        }
        for dir in next_dirs(pos, &passcode, &path) {
            let new_path = format!("{}{}", path, dir.as_char());
            let new_pos = dir.step(pos);
            queue.push_back((new_pos, new_path));
        }
    }
}

fn solve_part1(input_text: String) -> String {
    bfs_search_shortest(input_text)
}

fn bfs_search_longest(passcode: String) -> String {
    const START: Point2d = (0, 0);
    const TARGET: Point2d = (3, 3);
    let mut queue: VecDeque<(Point2d, String)> = VecDeque::new();
    queue.push_back((START, "".to_string()));
    let mut longest_path: String = "".to_string();

    let mut iterations: u32 = 0;
    loop {
        if queue.is_empty() {
            break;
        }
        let (pos, path) = queue
            .pop_front()
            .expect("error, queue cannot be empty here");
        iterations += 1;
        if iterations % 4096 == 0 {
            println!(
                "it {}, queue len {}, cur pos {:?}, cur steps {}",
                iterations,
                queue.len(),
                pos,
                path
            );
        }
        if pos == TARGET {
            longest_path = path;
            continue;
        }
        for dir in next_dirs(pos, &passcode, &path) {
            let new_path = format!("{}{}", path, dir.as_char());
            let new_pos = dir.step(pos);
            queue.push_back((new_pos, new_path));
        }
    }
    longest_path
}

fn solve_part2(input_text: String) -> String {
    bfs_search_longest(input_text).len().to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 17)
            .expect("Could not fetch input").trim().to_string();
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("hijkl")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digest() {
        assert_eq!(digest("hijkl", &"".to_string()), "ced9");
        assert_eq!(digest("hijklD", &"".to_string()), "f2bc");
        assert_eq!(digest("hijklDR", &"".to_string()), "5745");
        assert_eq!(digest("hijklDU", &"".to_string()), "528e");
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("ihgpwlah".to_string()), "DDRRRD");
        assert_eq!(solve_part1("kglvqrro".to_string()), "DDUDRLRRUDRD");
        assert_eq!(solve_part1("ulqzkmiv".to_string()), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }
}
