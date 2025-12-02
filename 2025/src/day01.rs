use crate::helper;

fn input_to_numbers(input_text: String) -> Vec<i16> {
    let lines = input_text.split("\n").collect::<Vec<&str>>();
    let words = lines
        .iter()
        .map(|x| x.trim().replace("L", "-").replace("R", ""))
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();
    words
        .iter()
        .map(|x| x.parse::<i16>().unwrap())
        .collect::<Vec<i16>>()
}

fn solve_part1(input_text: String) -> String {
    let numbers = input_to_numbers(input_text);
    let dial_max = 100;
    let mut dial = 50;
    let mut zeros = 0;
    for step in numbers {
        dial = (dial + step) % dial_max;
        if dial == 0 {
            zeros += 1;
        }
        //println!("step: {}", step);
    }

    zeros.to_string()
}

fn solve_part2(input_text: String) -> String {
    let numbers = input_to_numbers(input_text);
    let dial_max = 100;
    let mut dial = 50;
    let mut zeros = 0;
    for step in numbers {
        let full_rotations = step.abs() / dial_max;
        let partial_rotations = step % dial_max;
        zeros += full_rotations;

        let mut new_dial = dial + partial_rotations;
        if new_dial == 0 {
            zeros += 1;
        } else if new_dial >= dial_max {
            new_dial -= dial_max;
            zeros += 1;
        } else if new_dial < 0 {
            new_dial += dial_max;
            if dial > 0 {
                zeros += 1;
            }
        }

        // println!("step {}: {} -> {}  (z {})", step, dial, new_dial, zeros);
        dial = new_dial;
    }

    zeros.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2025, 1).expect("Could not fetch input");
    let _ = example_text();

    print!(
        "Part 1: {}\nPart 2: {}\n",
        solve_part1(input_text.clone()),
        solve_part2(input_text)
    );
}

fn example_text() -> String {
    String::from(
        "L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "3");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "6");
    }
}
