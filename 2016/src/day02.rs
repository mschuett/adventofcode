use crate::helper;
use crate::coord2d::Coord2d;


enum Direction {
    U, R, D, L
}
impl Direction {
    pub fn from(input: char) -> Direction {
        match input {
            'U' => Direction::U,
            'R' => Direction::R,
            'D' => Direction::D,
            'L' => Direction::L,
            _ => todo!(),
        }
    }
}

struct Keypad {
    keys: [[char; 3]; 3],
    pos: Coord2d,
}
impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: [
                ['1', '2', '3'],
                ['4', '5', '6'],
                ['7', '8', '9'],
            ],
            pos: Coord2d { x: 1, y: 1 } }
            // (0,0) -> key 1, (1,0) -> key 2, etc
    }
    pub fn get_key(&self) -> char {
        assert!(self.pos.x <= 2 && self.pos.y <= 2);
        self.keys[self.pos.y as usize][self.pos.x as usize]
    }
    fn step(&mut self, dir: Direction) {
        match dir {
            Direction::U => if self.pos.y > 0 { self.pos.y -= 1 }
            Direction::D => if self.pos.y < 2 { self.pos.y += 1 }
            Direction::L => if self.pos.x > 0 { self.pos.x -= 1 }
            Direction::R => if self.pos.x < 2 { self.pos.x += 1 }
        }
    }
}

struct Keypad2 {
    keys: [[char; 5]; 5],
    pos: Coord2d,
}

impl Keypad2 {
    pub fn new() -> Keypad2 {
        Keypad2 {
            keys: [
                [' ', ' ', '1', ' ', ' '],
                [' ', '2', '3', '4', ' '],
                ['5', '6', '7', '8', '9'],
                [' ', 'A', 'B', 'C', ' '],
                [' ', ' ', 'D', ' ', ' '],
            ],
            pos: Coord2d { x: 0, y: 2 } }  // still start at '5'
    }
    pub fn get_key(&self) -> char {
        self.keys[self.pos.y as usize][self.pos.x as usize]
    }
    fn step(&mut self, dir: Direction) {
        match dir {
            Direction::U => if self.pos.y > 0 && self.keys[(self.pos.y - 1) as usize][self.pos.x as usize] != ' ' { self.pos.y -= 1 }
            Direction::D => if self.pos.y < 4 && self.keys[(self.pos.y + 1) as usize][self.pos.x as usize] != ' ' { self.pos.y += 1 }
            Direction::L => if self.pos.x > 0 && self.keys[self.pos.y as usize][(self.pos.x - 1) as usize] != ' ' { self.pos.x -= 1 }
            Direction::R => if self.pos.x < 4 && self.keys[self.pos.y as usize][(self.pos.x + 1) as usize] != ' ' { self.pos.x += 1 }
        }
    }
}

fn solve_part1(input_text: String) -> String {
    let mut keypad = Keypad::new();
    let mut digits: Vec<char> = vec![];
    for line in input_text.lines() {
        for char in line.chars() {
            let dir = Direction::from(char);
            keypad.step(dir);
        }
        digits.push(keypad.get_key());
    }
    digits.iter().collect::<String>()
}

fn solve_part2(input_text: String) -> String {
    let mut keypad = Keypad2::new();
    let mut digits: Vec<char> = vec![];
    for line in input_text.lines() {
        for char in line.chars() {
            let dir = Direction::from(char);
            keypad.step(dir);
        }
        digits.push(keypad.get_key());
    }
    digits.iter().collect::<String>()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 2)
        .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from(
        "ULL\nRRDDD\nLURDL\nUUUUD\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "1985");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "5DB3");
    }
}
