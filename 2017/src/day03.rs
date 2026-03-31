use crate::helper;

fn get_ring(i: u32) -> u32 {
    let mut ring: u32 = 0;
    let mut ring_max: u32 = 0;
    while ring_max < i {
        ring += 1;
        ring_max = (2 * ring + 1).pow(2);
    }
    ring
}

fn ring_steps(i: u32) -> u32 {
    // the lower right number (=largest in "ring n") is (2n+1)^2
    // => first find the ring, then the position inside it
    let ring: u32 = get_ring(i);

    let ring_max: u32 = (2 * ring + 1).pow(2);
    let ring_lower_left: u32 = ring_max - 2*ring;
    let ring_upper_left: u32 = ring_lower_left - 2*ring;
    let ring_upper_right: u32 = ring_upper_left - 2*ring;
    let ring_lower_right: u32 = ring_upper_right - 2*ring;

    println!("i={}; in ring={} (max {}) with corners {}, {}, {}, and {}",
        i, ring, ring_max,
        ring_lower_left, ring_upper_left, ring_upper_right, ring_lower_right);

    if i >= ring_lower_left {  // "bottom edge"
        let mid = ring_lower_left + ring;
        return ring + mid.abs_diff(i);
    }
    if i >= ring_upper_left {  // left edge
        let mid = ring_upper_left + ring;
        return ring + mid.abs_diff(i);
    }
    if i >= ring_upper_right {  // upper edge
        let mid = ring_upper_right + ring;
        return ring + mid.abs_diff(i);
    }
    // else: on the right hand side/edge of the square
    // here we also have to handle the lower right corner transition between two rings
    let mid = ring_lower_right + ring;
    ring + mid.abs_diff(i)
}

fn solve_part1(input_text: String) -> String {
    let input = u32::from_str_radix(&input_text.trim(), 10).unwrap();
    ring_steps(input).to_string()
}

const SIZE: usize = 512;
const CENTER: usize = SIZE / 2;

fn set_cell(grid: &mut Vec<[u32; SIZE]>, x: usize, y: usize) {
    grid[x][y] = grid[x-1][y-1] + grid[x-1][y] + grid[x-1][y+1]
                + grid[x][y-1]                 + grid[x][y+1]
                + grid[x+1][y-1] + grid[x+1][y] + grid[x+1][y+1];
    // println!("({},{}) -> {}",
    //          x as i32 - CENTER as i32,
    //          y as i32 - CENTER as i32,
    //          grid[x][y]);
}

fn generate_grid_up_to(stop_at: u32) -> u32 {
    let mut grid= vec![[0; SIZE]; SIZE];

    let mut x = CENTER;
    let mut y = CENTER;
    grid[x][y] = 1;
    // start first ring
    x += 1;
    set_cell(&mut grid, x, y);
    loop {
        // right side upward
        while grid[x-1][y] != 0 {
            y -= 1;
            set_cell(&mut grid, x, y);
            if (grid[x][y] > stop_at) {
                return grid[x][y]
            }
        }
        // upper side to the left
        while grid[x][y+1] != 0 {
            x -= 1;
            set_cell(&mut grid, x, y);
            if (grid[x][y] > stop_at) {
                return grid[x][y]
            }
        }
        // left edge downward
        while grid[x+1][y] != 0 {
            y += 1;
            set_cell(&mut grid, x, y);
            if (grid[x][y] > stop_at) {
                return grid[x][y]
            }
        }
        // lower side to the right
        while grid[x][y-1] != 0 {
            x += 1;
            set_cell(&mut grid, x, y);
            if (grid[x][y] > stop_at) {
                return grid[x][y]
            }
        }
    }
}

fn solve_part2(input_text: String) -> String {
    let input = u32::from_str_radix(&input_text.trim(), 10).unwrap();
    generate_grid_up_to(input).to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 3)
            .expect("Could not fetch input");

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(ring_steps(1), 0);
        assert_eq!(ring_steps(12), 3);
        assert_eq!(ring_steps(23), 2);
        assert_eq!(ring_steps(1024), 31);
    }
    #[test]

    fn test_part1_edge_cases() {
        assert_eq!(ring_steps(24), 3);
        assert_eq!(ring_steps(25), 4);
        assert_eq!(ring_steps(26), 5);
        assert_eq!(ring_steps(27), 4);
    }
}
