use crate::helper;

fn line_digits_to_vec_u8 (line: &str) -> Vec<u8> {
    let mut array: Vec<u8> = Vec::new();
    for c in line.chars() {
        array.push(c.to_digit(10).unwrap() as u8);
    }
    array
}

fn parse_input(input_text: &str) -> Vec<Vec<u8>> {
    let lines = input_text.trim().split("\n")
        .collect::<Vec<&str>>();
    let bytelines = lines
        .into_iter()
        .map(line_digits_to_vec_u8)
        .collect::<Vec<Vec<u8>>>();
    bytelines
}

fn solve_part1(input_text: String) -> String {
    let banks = parse_input(&input_text);

    let mut result_nums: Vec<u32> = Vec::new();
    for bank in banks {
        let bank_len = bank.len();

        // max_by() does not give me the first (=leftmost) element, hence use this to get the max
        // also: just ignore the last value, it can never become the first digit
        let max_val = bank[..bank_len-1].iter().max().unwrap();
        let max_pos = bank[..bank_len-1].iter().position(|v| v == max_val).unwrap();
        // print!("{bank:?} (len {}) -> max {max_pos:?},{max_val:?}", bank_len);

        // find 2nd max digit to the right of max_pos
        let max2_val = bank[max_pos+1..].iter().max().unwrap();
        // print!(" -> max2 {max2_val:?}");

        // now our number is <max,max2>
        let bank_result = ((max_val * 10) + max2_val) as u32;

        result_nums.push(bank_result);
        // println!(" -> {:?}", bank_result);
    }

    let result_sum: u32 = result_nums.iter().sum::<u32>();
    result_sum.to_string()
}

fn solve_part2(input_text: String) -> String {
    let banks = parse_input(&input_text);
    const DIGITS: usize = 12;
    let mut result_nums: Vec<u64> = Vec::new();

    for bank in banks {
        let bank_len = bank.len();
        let mut bank_max_digits: [u8; DIGITS] = [0; DIGITS];
        let mut bank_cursor: usize = 0;
        // print!("{bank:?} (len {})", bank_len);

        // generalize the max search from part 1 to multiple digits in changing search slices
        for i in 0..DIGITS {
            let limit_right = bank_len - DIGITS + i + 1;
            let max_val = bank[bank_cursor..limit_right].iter().max().unwrap();
            let max_pos = bank[bank_cursor..limit_right].iter().position(|v| v == max_val).unwrap();
            bank_max_digits[i] = *max_val;
            bank_cursor = bank_cursor+max_pos+1;
        }

        let mut bank_result: u64 = 0;
        for i in 0..DIGITS {
            let factor = 10u64.pow((DIGITS-i-1) as u32);
            bank_result += factor * bank_max_digits[i] as u64;
        }
        result_nums.push(bank_result);
        // println!(" -> {:?} {:?}", bank_max_digits, bank_result);
    }

    let result_sum: u64 = result_nums.iter().sum::<u64>();
    result_sum.to_string()
}


pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2025, 3).expect("Could not fetch input");
    let _ = example_text();

    print!(
        "Part 1: {}\nPart 2: {}\n",
        solve_part1(input_text.clone()),
        solve_part2(input_text)
    );
}

fn example_text() -> String {
    String::from(
"987654321111111
811111111111119
234234234234278
818181911112111",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "357");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "3121910778619");
    }
}
