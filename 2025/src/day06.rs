use crate::helper;

fn parse_input(input_text: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut lines = input_text
        .lines()
        .collect::<Vec<&str>>();
    let ops = lines
        .pop().unwrap()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<Vec<char>>();

    let mut num_lines: Vec<Vec<u64>> = Vec::new();
    for line in lines {
        let num_line = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        num_lines.push(num_line);
    };

    (num_lines, ops)
}

fn solve_part1(input_text: String) -> String {
    let (num_lines, ops) = parse_input(&input_text);

    let mut result: u64 = 0;
    for i in 0..ops.len() {
        let mut row_numbers: Vec<u64> = Vec::new();
        for num_line in &num_lines {
            row_numbers.push(num_line[i]);
        }
        let group_result: u64 = match ops[i] {
            '+' => { row_numbers.iter().sum::<u64>() },
            '*' => { row_numbers.iter().product::<u64>() }
            _   => { panic!("unexpected operator {}", ops[i]) }
        };
        result += group_result;
    }
    result.to_string()
}

fn parse_input_columns(input_text: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut lines = input_text
        .lines()
        .collect::<Vec<&str>>();
    let ops = lines
        .pop().unwrap()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<Vec<char>>();

    let mut input_nums: Vec<Vec<u64>> = Vec::new();
    let mut group_nums: Vec<u64> = Vec::new();
    // now rotate lines (without ops) counter-clockwise = read vertically
    for col in 0..lines[0].len() {
        let mut col_chars: Vec<char> = Vec::new();
        for line in &lines {
            let c: char = line.chars().nth(col).unwrap();
            col_chars.push(c);
        }
        let col_str = col_chars.iter().collect::<String>();
        if !col_str.trim().is_empty() {
            let col_num = col_str.trim().parse::<u64>().unwrap();
            group_nums.push(col_num);
        } else {
            // if whole string is ' ' then we have a full group and start a new one
            input_nums.push(group_nums);
            group_nums = Vec::new();
        }
    }
    // push last group
    input_nums.push(group_nums);
    (input_nums, ops)
}

fn solve_part2(input_text: String) -> String {
    let (input_nums, ops) = parse_input_columns(&input_text);

    let mut result: u64 = 0;
    for i in 0..ops.len() {
        let group_numbers = &input_nums[i];
        let group_result: u64 = match ops[i] {
            '+' => { group_numbers.iter().sum::<u64>() },
            '*' => { group_numbers.iter().product::<u64>() }
            _   => { panic!("unexpected operator {}", ops[i]) }
        };
        // println!("{} {:?} = {}", ops[i], group_numbers, group_result);
        result += group_result;
    }
    result.to_string()
}


pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2025, 6).expect("Could not fetch input");
    let _ = example_text();

    print!(
        "Part 1: {}\nPart 2: {}\n",
        solve_part1(input_text.clone()),
        solve_part2(input_text)
    );
}


fn example_text() -> String {
    String::from("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "4277556");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "3263827");
    }
}
