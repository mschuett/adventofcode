use crate::helper;

#[derive(Clone, Debug)]
struct SpinLock {
    values: Vec<u32>,
    position: usize,
}
impl SpinLock {
    fn new() -> Self {
        SpinLock { values: vec![0], position: 0 }
    }
    fn add_element(&mut self, skips: usize, newvalue: u32) {
        let newpos = ((self.position + skips) % self.values.len()) + 1;
        self.position = newpos;
        self.values.insert(newpos, newvalue);
    }
}

fn solve_part1(input_text: String) -> String {
    const LAST_VALUE: u32 = 2017;
    let skip_steps = input_text.trim().parse::<usize>().unwrap();
    let mut sl = SpinLock::new();
    for item in 1..=LAST_VALUE {
        sl.add_element(skip_steps, item);
        // println!("{:?}", sl);
    }
    sl.values[(sl.position+1) % sl.values.len()].to_string()
}

// for part 2 do not keep the values in memory,
// only the relevant state such as the size, the current position, and the value next to 0
#[derive(Clone, Debug)]
struct SpinLock2 {
    position: usize,
    size: usize,
    pos_of_zero: usize,
    value_after_zero: u32,
}
impl SpinLock2 {
    fn new() -> Self {
        SpinLock2 { position: 0, size: 1, pos_of_zero: 0, value_after_zero: 0 }
    }
    fn add_element(&mut self, skips: usize, newvalue: u32) {
        let newpos = ((self.position + skips) % self.size) + 1;
        self.position = newpos;
        if newpos == self.pos_of_zero {
            // insert here will move the 0 to the right
            self.pos_of_zero += 1;
        } else if newpos == self.pos_of_zero + 1 {
            // this sets the value after the 0
            self.value_after_zero = newvalue;
        }
        self.size += 1;
    }
}

fn solve_part2(input_text: String) -> String {
    const LAST_VALUE: u32 = 50000000;
    let skip_steps = input_text.trim().parse::<usize>().unwrap();
    let mut sl = SpinLock2::new();
    for item in 1..=LAST_VALUE {
        sl.add_element(skip_steps, item);
        // println!("{:?}", sl);
    }
    sl.value_after_zero.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 17)
            .expect("Could not fetch input");
    println!("Example: {}", solve_part1(example_text()));
    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("3")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "638");
    }
}
