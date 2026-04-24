use std::collections::HashMap;
use crate::{day10, day12, helper};

fn knot_hash(input_text: &String) -> String {
    day10::solve_part2(input_text.trim().to_string())
}

fn solve_part1(input_text: String) -> String {
    let mut count_bin_ones: u32 = 0;
    let hash_inputs = (0..128)
        .map(|n| format!("{}-{}", input_text.trim(), n))
        .collect::<Vec<String>>();
    let hashes = hash_inputs.iter()
        .map(knot_hash)
        .collect::<Vec<String>>();
    for line in hashes {
        let line_number: u128 = u128::from_str_radix(&line, 16).unwrap();
        count_bin_ones += line_number.count_ones();
    }
    count_bin_ones.to_string()
}

fn is_bit_set(num: u128, bit: u8) -> bool {
    num & (1 << bit) != 0
}

fn solve_part2(input_text: String) -> String {
    let hash_inputs = (0..128)
        .map(|n| format!("{}-{}", input_text.trim(), n))
        .collect::<Vec<String>>();
    let hashes = hash_inputs.iter()
        .map(knot_hash)
        .collect::<Vec<String>>();
    // re-use group count from day 12 => build connection map
    let mut connection_map: HashMap<u16, Vec<u16>> = HashMap::new();

    fn pos_id(line: u8, bit: u8) -> u16 {
        // unique id for every bit position in the 128x128 grid
        ((line as u16) << 8) + bit as u16
    }
    let mut prev_line: u128 = 0;
    for (y, line) in hashes.iter().enumerate() {
        let line_number: u128 = u128::from_str_radix(line, 16).unwrap();
        let mut bit: u8 = 0;
        while bit < u128::BITS as u8 {
            if !is_bit_set(line_number, bit) {
                bit += 1;
                continue;
            }
            // start a new map entry
            let start_num = pos_id(y as u8, bit);
            let mut connected_nums: Vec<u16> = vec![start_num];
            if is_bit_set(prev_line, bit) {
                assert!(y > 0);
                connected_nums.push(pos_id((y as u8) - 1, bit));
            }
            bit += 1;
            // collect the full sequence of multiple set bits
            while bit < u128::BITS as u8 && is_bit_set(line_number, bit) {
                connected_nums.push(pos_id(y as u8, bit));
                if is_bit_set(prev_line, bit) {
                    connected_nums.push(pos_id((y as u8) - 1, bit));
                }
                bit += 1;
            }
            connection_map.insert(start_num, connected_nums);
        }
        prev_line = line_number;
    }
    day12::find_group_count(&connection_map).to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 14)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("flqrgnkx")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knot_hash() {
        // check the import
        assert_eq!(knot_hash(&"".to_string()), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(knot_hash(&"AoC 2017".to_string()), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(knot_hash(&"1,2,3".to_string()), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(knot_hash(&"1,2,4".to_string()), "63960835bcdc130f0b66d7ff4f6a5a8e");
        // check with data from reddit
        assert_eq!(knot_hash(&"flqrgnkx-0".to_string()), "d4f76bdcbf838f8416ccfa8bc6d1f9e6");
        assert_eq!(knot_hash(&"flqrgnkx-1".to_string()), "55eab3c4fbfede16dcec2c66dda26464");
    }

    #[test]
    fn test_knot_hash_bits() {
        let bits0 = format!("{:08b}", u8::from_str_radix(
            &knot_hash(&"flqrgnkx-0".to_string())[0..2], 16).unwrap());
        assert_eq!(bits0, "11010100");
        let bits1 = format!("{:08b}", u8::from_str_radix(
            &knot_hash(&"flqrgnkx-1".to_string())[0..2], 16).unwrap());
        assert_eq!(bits1, "01010101");
        let bits2 = format!("{:08b}", u8::from_str_radix(
            &knot_hash(&"flqrgnkx-2".to_string())[0..2], 16).unwrap());
        assert_eq!(bits2, "00001010");
    }

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "8108");
    }

    #[test]
    fn test_part2_is_bit_set() {
        assert!(!is_bit_set(0, 0));
        assert!(!is_bit_set(0, 1));
        assert!(!is_bit_set(0, 2));
        assert!(!is_bit_set(0, 3));
        assert!(is_bit_set(1, 0));
        assert!(is_bit_set(2, 1));
        assert!(is_bit_set(1024, 10));
        assert!(!is_bit_set(1024, 9));
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "1242");
    }
}
