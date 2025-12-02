use crate::helper;
use std::collections::HashSet;

fn parse_input(input_text: String) -> Vec<(u64, u64)> {
    let lines = input_text.split("\n")
        .collect::<Vec<&str>>();
    let line = lines.first().unwrap().split(',')
        .collect::<Vec<&str>>();
    let str_pairs = line.into_iter()
        .map(|p| p.split_once('-').unwrap())
        .collect::<Vec<(&str,&str)>>();
    let num_pairs = str_pairs.into_iter()
        .map(|(a,b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect::<Vec<(u64,u64)>>();
    num_pairs
}

fn solve_part1(input_text: String) -> String {
    let pairs = parse_input(input_text);
    let mut result_nums: Vec<u64> = Vec::new();

    // trying a numeric approach
    for (low,high) in pairs {
        let mut found_new_nums: Vec<u64> = Vec::new();
        for i in low..high+1 {
            let digits = i.checked_ilog10().unwrap()+1;
            if digits % 2 == 1 {
                continue
            }
            let mod_num = 10u64.pow(digits/2);
            let left_part = i / mod_num;
            let right_part = i % mod_num;
            if left_part == right_part {
                found_new_nums.push(i);
            }
        }
        result_nums.append(&mut found_new_nums);
    }
    let result_sum = result_nums.iter().sum::<u64>();
    result_sum.to_string()
}

fn solve_part2(input_text: String) -> String {
    let pairs = parse_input(input_text);
    let mut result_nums: Vec<u64> = Vec::new();

    // using strings
    for (low,high) in pairs {
        let mut found_new_nums: HashSet<u64> = HashSet::new();
        for i in low..high+1 {
            let istr = i.to_string();
            for slicelen in 1..istr.len() {
                if istr.len() % slicelen != 0 {
                    continue
                }
                let pattern = istr[..slicelen].to_string();
                let chunks = istr.len()/slicelen;

                let mut matched = true;
                for chunk in 1..chunks {
                    let offset_start = chunk*slicelen;
                    let offset_end = (chunk+1)*slicelen;
                    if istr[offset_start..offset_end] != pattern {
                        matched = false;
                        break
                    }
                }
                if matched {
                    // println!("{}-{}, i {}, slicelen {} found it", low, high, i, slicelen);
                    found_new_nums.insert(i);
                }
            }
        }
        result_nums.append(&mut found_new_nums.into_iter().collect());
    }
    let result_sum = result_nums.iter().sum::<u64>();
    result_sum.to_string()
}


pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2025, 2).expect("Could not fetch input");
    let _ = example_text();

    print!(
        "Part 1: {}\nPart 2: {}\n",
        solve_part1(input_text.clone()),
        solve_part2(input_text)
    );
}

fn example_text() -> String {
    String::from(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "1227775554");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "4174379265");
    }
}
