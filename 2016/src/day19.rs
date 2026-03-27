use crate::helper;

// https://en.wikipedia.org/wiki/Josephus_problem#k_=_2
fn get_winner(n: u32) -> u32 {
    let firstbit = 1<<(u32::BITS-n.leading_zeros()-1);
    let l = n - firstbit;
    2*l+1
}

fn solve_part1(input_text: String) -> String {
    get_winner(input_text.trim().parse::<u32>().unwrap()).to_string()
}

fn simulate_circle(n: u32) -> u32 {
    let mut alive: u32 = n;
    let mut circle: Vec<u32> = (1..=n).collect();
    let mut current: u32 = 0;

    while alive > 1 {
        let mut skip_to_kill = alive / 2;
        let mut kill_target = current;
        while skip_to_kill > 0 {
            kill_target = (kill_target + 1) % n;
            if circle[kill_target as usize] != 0 {
                skip_to_kill -= 1;
            }
        }
        circle[kill_target as usize] = 0;
        alive -= 1;
        loop {  // go to next available
            current = (current + 1) % n;
            if circle[current as usize] != 0 { break; }
        }
    }
    circle[current as usize]
}

// https://oeis.org/A334473
fn highest_power_of_three(n: u32) -> u32 {
    let mut x = 0;
    while 3u32.pow(x) <= n {
        x += 1;
    }
    3u32.pow(x - 1)
}
fn circle_winner(n: u32) -> u32 {
    let x = highest_power_of_three(n);
    if x == n {
        x
    } else {
        if n < 2 * x {
            n % x
        } else {
            x + 2 * (n % x)
        }
    }
}

fn solve_part2(input_text: String) -> String {
    circle_winner(input_text.trim().parse::<u32>().unwrap()).to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 19)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("5")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_winner() {
        assert_eq!(get_winner(1), 1);
        assert_eq!(get_winner(2), 1);
        assert_eq!(get_winner(3), 3);
        assert_eq!(get_winner(4), 1);
        assert_eq!(get_winner(5), 3);
        assert_eq!(get_winner(6), 5);
        assert_eq!(get_winner(7), 7);
        assert_eq!(get_winner(8), 1);
        assert_eq!(get_winner(9), 3);
        assert_eq!(get_winner(10), 5);
        assert_eq!(get_winner(11), 7);
        assert_eq!(get_winner(12), 9);
        assert_eq!(get_winner(13), 11);
        assert_eq!(get_winner(14), 13);
        assert_eq!(get_winner(15), 15);
    }

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "3");
    }

    #[test]
    fn test_part2() {
        for i in 1..100 {
            assert_eq!(simulate_circle(i), circle_winner(i));
        }
    }
}
