use crate::helper;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct FirewallLayer {
    depth: u8,
    range: u8,
}
impl TryFrom<&str> for FirewallLayer {
    type Error = String;
    fn try_from(s: &str) -> Result<FirewallLayer, Self::Error> {
        let (depth_str, range_str) = s.trim().split_once(": ").unwrap();
        let depth = depth_str.parse::<u8>().unwrap();
        let range = range_str.parse::<u8>().unwrap();
        Ok(FirewallLayer{depth, range})
    }
}
impl FirewallLayer {
    fn scans_at_zero_at_picos(&self, picos: u32) -> bool {
        let cycle_len = 2 * (self.range as u32 - 1);
        picos.is_multiple_of(cycle_len)
    }
}

fn solve_part1(input_text: String) -> String {
    let mut layers: Vec<FirewallLayer> = input_text.lines()
        .map(FirewallLayer::try_from)
        .rev()
        .collect::<Result<_, _>>()
        .unwrap();
    let mut cur_score: u32 = 0;
    while let Some(cur_layer) = layers.pop() {
        if cur_layer.scans_at_zero_at_picos(cur_layer.depth as u32) {
            // println!("caught in layer {}", cur_layer.depth);
            cur_score += cur_layer.depth as u32 * cur_layer.range as u32;
        }
    }
    cur_score.to_string()
}

fn solve_part2(input_text: String) -> String {
    let layers: Vec<FirewallLayer> = input_text.lines()
        .map(FirewallLayer::try_from)
        .rev()
        .collect::<Result<_, _>>()
        .unwrap();

    // feels like brute force; i assume there is an analytical solution as well :(
    for delay in 0..u32::MAX {
        if layers.iter().any(|l|
            l.scans_at_zero_at_picos(l.depth as u32 + delay)
        ) {
            continue
        }
        else {
            return delay.to_string();
        }
    }
    unreachable!("cannot find any valid delay")
}


pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 13)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("\
0: 3
1: 2
4: 4
6: 4
")}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "24");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "10");
    }
}
