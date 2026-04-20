use crate::helper;

struct Generator {
    previous: u32,
    factor: u32,
    modulo: u32,
}
impl Iterator for Generator {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let new_value = ((self.previous as u64 * self.factor as u64) % self.modulo as u64) as u32;
        self.previous = new_value;
        Some(new_value)
    }
}

const GEN_MODULO: u32 = 2147483647;
const GEN_A_FACTOR: u32 = 16807;
const GEN_B_FACTOR: u32 = 48271;

fn judge(a: &mut dyn Iterator<Item=u32>, b: &mut dyn Iterator<Item=u32>, pairs: u32) -> u32 {
    let mut match_count = 0u32;
    for _ in 0..pairs {
        let low16_a = a.next().unwrap() as u16;
        let low16_b = b.next().unwrap() as u16;
        if low16_a == low16_b {
            match_count += 1;
        }
    }
    match_count
}

fn parse_input_seeds(input: String) -> Vec<u32> {
    input.lines()
        .map(|line| line
            .rsplit_once(' ').unwrap()
            .1.parse::<u32>().unwrap())
        .collect()
}

fn solve_part1(input_text: String) -> String {
    let seeds: Vec<u32> = parse_input_seeds(input_text);
    let mut a = Generator{previous: seeds[0], factor: GEN_A_FACTOR, modulo: GEN_MODULO};
    let mut b = Generator{previous: seeds[1], factor: GEN_B_FACTOR, modulo: GEN_MODULO};
    judge(&mut a, &mut b, 40_000_000).to_string()
}

const GEN_A_CRITERIUM: u32 = 4;
const GEN_B_CRITERIUM: u32 = 8;

fn solve_part2(input_text: String) -> String {
    let seeds: Vec<u32> = parse_input_seeds(input_text);
    let mut a = Generator{previous: seeds[0], factor: GEN_A_FACTOR, modulo: GEN_MODULO}
        .filter(|x| x.is_multiple_of(GEN_A_CRITERIUM));
    let mut b = Generator{previous: seeds[1], factor: GEN_B_FACTOR, modulo: GEN_MODULO}
        .filter(|x| x.is_multiple_of(GEN_B_CRITERIUM));
    judge(&mut a, &mut b, 5_000_000).to_string()
}


pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 15)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("\
Generator A starts with 65
Generator B starts with 8921
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "588");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "309");
    }
}
