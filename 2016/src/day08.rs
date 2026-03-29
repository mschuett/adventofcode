use crate::helper;
use std::fmt;

#[derive(Debug,PartialEq)]
struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<Vec<bool>>,
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = self.pixels
            .iter()
            .map(|x|
                x.iter()
                    .map(| b | match b {
                        true => '#',
                        false => '.',
                    })
                    .collect::<String>()
            )
            .collect::<Vec<String>>()
        .join("\n");
        write!(f, "{}\n", output)
    }
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {
        let pixels = vec![vec![false; width]; height];
        Screen {width, height, pixels}
    }
    pub fn op(&mut self, line: &str) {
        let (command, args) = line.split_once(' ').unwrap();
        match command {
            "rect" => {
                let (x_str,y_str) = args
                    .trim()
                    .split_once('x').unwrap();
                let x = x_str.parse::<usize>().unwrap();
                let y = y_str.parse::<usize>().unwrap();
                self.rect(x, y);
            }
            "rotate" => {
                let rot_args = args.split_whitespace().collect::<Vec<&str>>();
                assert_eq!(4, rot_args.len());
                assert_eq!("by", rot_args[2]);
                let shift_count = rot_args[3].parse::<usize>().unwrap();
                let row_or_col = rot_args[1]
                    .split_once('=').unwrap()
                    .1
                    .parse::<usize>().unwrap();
                match rot_args[0] {
                    "column" => self.rotate_column(row_or_col, shift_count),
                    "row" => self.rotate_row(row_or_col, shift_count),
                    _ => unimplemented!("unknown rotate op: {}", args)
                }
            }
            _ => unimplemented!("unknown command {}", command),
        }
    }
    fn rect(&mut self, a: usize, b: usize) {
        for y in 0..b {
            for x in 0..a {
                self.pixels[y][x] = true;
            }
        }
    }
    fn rotate_row(&mut self, y: usize, by: usize) {
        assert!(y < self.height);
        self.pixels[y].rotate_right(by)
    }
    fn rotate_column(&mut self, x: usize, by: usize) {
        assert!(x < self.width);
        let mut column = vec![false; self.height];
        for y in 0..self.height {
            column[y] = self.pixels[y][x];
        }
        column.rotate_right(by);
        for y in 0..self.height {
            self.pixels[y][x] = column[y];
        }
    }
    pub fn count_pixels(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.pixels[y][x] {
                    count += 1;
                }
            }
        }
        count
    }
}

fn solve_part1(input_text: String) -> String {
    let mut screen = Screen::new(50, 6);
    for line in input_text.lines() {
        screen.op(line);
        println!("{}:\n{}", line, screen);
    }
    screen.count_pixels().to_string()
}

// part 2 is reading the screen display, not in code

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 8)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text));
}

fn example_text() -> String {
    String::from("rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut screen = Screen::new(7, 3);
        for line in example_text().lines() {
            screen.op(line);
        }
        let result = screen.count_pixels();
        assert_eq!(result, 6);
    }

    fn test_rot_col() {
        let mut screen = Screen::new(7, 3);
        screen.op("rect 3x1");
        for i in 0..50 {
            screen.op(&format!("rotate column x=1 by {}", i));
            assert_eq!(screen.count_pixels(), 3);
        }
    }

    fn test_rot_row() {
        let mut screen = Screen::new(7, 3);
        screen.op("rect 3x1");
        for i in 0..50 {
            screen.op(&format!("rotate row y=1 by {}", i));
            assert_eq!(screen.count_pixels(), 3);
        }
    }
}
