use crate::helper;

fn parse_input(input_text: &str) -> Vec<Vec<char>> {
    let lines = input_text.trim().split("\n")
        .collect::<Vec<&str>>();
    lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn count_grid_neighbours(grid: &[Vec<char>], x: usize, y: usize, mx: usize, my: usize) -> u8 {
    let mut ncount = 0;
    if y > 0    && x > 0    && grid[y-1][x-1] == '@' { ncount += 1;}
    if y > 0    &&             grid[y-1][x  ] == '@' { ncount += 1;}
    if y > 0    && x < mx-1 && grid[y-1][x+1] == '@' { ncount += 1;}

    if             x > 0    && grid[y  ][x-1] == '@' { ncount += 1;}
    if             x < mx-1 && grid[y  ][x+1] == '@' { ncount += 1;}

    if y < my-1 && x > 0    && grid[y+1][x-1] == '@' { ncount += 1;}
    if y < my-1 &&             grid[y+1][x  ] == '@' { ncount += 1;}
    if y < my-1 && x < mx-1 && grid[y+1][x+1] == '@' { ncount += 1;}
    ncount
}

fn solve_part1(input_text: String) -> String {
    let mut count = 0;
    let grid = parse_input(&input_text);

    let my = grid.len();
    let mx = grid[0].len();

    for y in 0..my {
        for x in 0..mx {
            if grid[y][x] != '@' {
                continue;  // empty
            }
            if count_grid_neighbours(&grid, x, y, mx, my) < 4 {
                count += 1
            };
        }
    }
    count.to_string()
}

fn solve_part2(input_text: String) -> String {
    let mut count = 0;
    let mut grid = parse_input(&input_text);
    let mut grid_changed = true;

    let my = grid.len();
    let mx = grid[0].len();

    while grid_changed {
        grid_changed = false;

        // get removals
        let mut removals: Vec<(usize, usize)> = Vec::new();
        for y in 0..my {
            for x in 0..mx {
                if grid[y][x] != '@' {
                    continue;  // empty
                }
                if count_grid_neighbours(&grid, x, y, mx, my) < 4 {
                    removals.push((x, y));
                };
            }
        }
        // remove items
        for (x, y) in removals {
            grid[y][x] = '.';
            count += 1;
            grid_changed = true;
        }
    }

    count.to_string()
}


pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2025, 4).expect("Could not fetch input");
    let _ = example_text();

    print!(
        "Part 1: {}\nPart 2: {}\n",
        solve_part1(input_text.clone()),
        solve_part2(input_text)
    );
}

fn example_text() -> String {
    String::from(
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "43");
    }
}
