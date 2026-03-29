use crate::helper;

type Tile = bool;

fn char_to_tile(c: char) -> Tile {
    c == '^'
}

fn string_to_tiles(line: String) -> Vec<Tile> {
    line.trim().chars().map(|c| char_to_tile(c)).collect()
}

fn tiles_to_string(line: Vec<Tile>) -> String {
    line.iter().map(|&t| if t {"^"} else {"."}).collect()
}

fn is_trap(a: Tile, b: Tile, c: Tile) -> bool {
       ( a &&  b && !c)
    || (!a &&  b &&  c)
    || ( a && !b && !c)
    || (!a && !b &&  c)
}

fn next_row(prev: Vec<Tile>) -> Vec<Tile> {
    let mut result: Vec<Tile> = vec![];

    result.push(is_trap(false, prev[0], prev[1]));
    for a in prev.windows(3) {
        result.push(is_trap(a[0], a[1], a[2]));
    }
    result.push(is_trap(prev[prev.len()-2], prev[prev.len() - 1], false));

    result
}

fn count_safe_tiles(first_row: String, rows: u32) -> u32 {
    let mut current_row = string_to_tiles(first_row);
    let mut count_safe_tiles: u32 = 0;

    for _ in 0..rows {
        count_safe_tiles += current_row.iter().filter(|&&x| { !x }).count() as u32;
        current_row = next_row(current_row);
    }
    count_safe_tiles
}

fn solve_example(input_text: String) -> String {
    count_safe_tiles(input_text, 10).to_string()
}

fn solve_part1(input_text: String) -> String {
    count_safe_tiles(input_text, 40).to_string()
}

fn solve_part2(input_text: String) -> String {
    count_safe_tiles(input_text, 400000).to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 18)
            .expect("Could not fetch input");
    println!("Example: {}", solve_example(example_text()));
    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from(".^^.^.^^^^")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_row() {
        let first_row = string_to_tiles("..^^.".to_string());
        assert_eq!(".^^^^", tiles_to_string(next_row(first_row)));

        let second_row = string_to_tiles(".^^^^".to_string());
        assert_eq!("^^..^", tiles_to_string(next_row(second_row)));
    }

    #[test]
    fn test_part1() {
        let result = solve_example(example_text());
        assert_eq!(result, "38");
    }
}
