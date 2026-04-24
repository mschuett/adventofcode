use crate::helper;

fn solve_part1(input_text: String) -> String {
    let mut jumps: Vec<i32> = input_text
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let mut pc: usize = 0;
    let mut steps: u32 = 0;
    while pc < jumps.len() {
        let newpc = (pc as i32 + jumps[pc]) as usize;
        jumps[pc] += 1;
        pc = newpc;
        steps += 1;
    }
    steps.to_string()
}

fn solve_part2(input_text: String) -> String {
    let mut jumps: Vec<i32> = input_text
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let mut pc: usize = 0;
    let mut steps: u32 = 0;
    while pc < jumps.len() {
        let newpc = (pc as i32 + jumps[pc]) as usize;
        if jumps[pc] >= 3 {
            jumps[pc] -= 1;
        } else {
            jumps[pc] += 1;
        }
        pc = newpc;
        steps += 1;
    }
    steps.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 5)
            .expect("Could not fetch input");

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("\
0
3
0
1
-3
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "5");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "10");
    }
}
