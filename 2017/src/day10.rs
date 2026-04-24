use itertools::Itertools;
use crate::helper;

struct CircularList {
    list: Vec<u8>,
    // to avoid "wrap-around" handling we always rotate the list,
    // so the current position is always list[0],
    // cur_rotation keeps track of this, used by reset_rotation()
    cur_pos: usize,
    cur_rotation: usize,
    skip_size: usize,
}
impl CircularList {
    fn new(size: usize) -> Self {
        let mut list: Vec<u8> = Vec::with_capacity(size);
        for i in 0..size {
            list.push(i as u8);
        }
        let cur_pos: usize = 0;
        let cur_rotation: usize = 0;
        let skip_size: usize = 0;
        CircularList{list, cur_pos, cur_rotation, skip_size}
    }
    fn single_round(&mut self, lengths: &Vec<u8>) {
        let size = self.list.len();
        for cur_length in lengths {
            self.list.rotate_left(self.cur_pos % size);
            self.cur_rotation += self.cur_pos;
            // redundant: self.cur_pos = 0;
            self.list[0..*cur_length as usize].reverse();
            self.cur_pos = *cur_length as usize + self.skip_size;
            self.skip_size += 1;
        }
    }
    fn reset_rotation(&mut self) {
        let size = self.list.len();
        self.list.rotate_right(self.cur_rotation % size);
    }
    fn dense_hash(&mut self) -> String {
        self.reset_rotation();
        let blocks = self.list
            .chunks(16)
            .map(|block| block.iter()
                .fold(0, |acc, &x| acc ^ x)
            )
            .collect::<Vec<u8>>();
        format!("{:02x}", blocks.iter().format(""))
    }
}

fn get_lengths_simple(inut_text: String) -> Vec<u8> {
    inut_text.trim()
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}
fn solve_part1(input_text: String, list_size: usize) -> String {
    let mut circlist = CircularList::new(list_size);
    let lenghts: Vec<u8> = get_lengths_simple(input_text);
    circlist.single_round(&lenghts);
    circlist.reset_rotation();
    (circlist.list[0] as u32 * circlist.list[1] as u32).to_string()
}

fn get_lengths_part2(input_text: String) -> Vec<u8> {
    let extra_vec: Vec<u8> = vec![17, 31, 73, 47, 23];
    let mut result = input_text.trim().as_bytes().to_vec();
    result.extend(extra_vec);
    result
}

pub fn solve_part2(input_text: String) -> String {
    let list_size = 256;
    let mut circlist = CircularList::new(list_size);
    let lenghts: Vec<u8> = get_lengths_part2(input_text);
    for round in 0..64 {
        circlist.single_round(&lenghts);
    }
    circlist.dense_hash()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 10)
            .expect("Could not fetch input");
    println!("Example: {}", solve_part1(example_text(), 5));
    println!("Part 1: {}", solve_part1(input_text.clone(), 256));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("3, 4, 1, 5")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text(), 5);
        assert_eq!(result, "12");
    }

    #[test]
    fn test_part2_get_lengths() {
        let result = get_lengths_part2("1,2,3".to_string());
        assert_eq!(result, vec![49,44,50,44,51,17,31,73,47,23]);
    }

    #[test]
    fn test_part2_hashes() {
        assert_eq!(solve_part2("".to_string()), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(solve_part2("AoC 2017".to_string()), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(solve_part2("1,2,3".to_string()), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(solve_part2("1,2,4".to_string()), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
