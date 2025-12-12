use crate::helper;

fn solve_part1(input_text: String) -> String {
    String::from("missing")
}

fn solve_part2(input_text: String) -> String {
    String::from("missing")
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2025, 1)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from(
        "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "0");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "0");
    }
}
