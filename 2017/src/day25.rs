use std::collections::{BTreeMap,HashMap};
use crate::helper;

type State = char;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Left, Right
}
// value, move, continue state
type Action = (bool, Direction, State);

#[derive(Debug)]
struct TuringMachine {
    current_step: u32,
    diagnostic_after: u32,
    current_state: State,
    tape: BTreeMap<i32, bool>,
    tape_pos: i32,
    rules: HashMap<(State, bool), Action>,
}
impl From<String> for TuringMachine {
    fn from(input_text: String) -> Self {
        let current_step: u32 = 0;
        let tape: BTreeMap<i32, bool> = BTreeMap::from([(0, false)]);
        let tape_pos: i32 = 0;

        let lines = input_text.lines().collect::<Vec<&str>>();
        let current_state = lines[0]
            .rsplit_once(' ').unwrap().1
            .chars().next().unwrap();
        assert!(current_state.is_ascii_uppercase());

        let diagnostic_after: u32 = lines[1].split(' ')
            .nth(5).unwrap()
            .parse().unwrap();

        let mut rules: HashMap<(State, bool), Action> = HashMap::new();
        let mut cur_line = 3;

        while cur_line < lines.len() {
            assert!(lines[cur_line].starts_with("In state "));
            let state = lines[cur_line]
                .rsplit_once(' ').unwrap().1
                .chars().next().unwrap() as State;
            cur_line += 1;
            for _ in 0..2 {  // two action instruction blocks
                assert!(lines[cur_line].starts_with("  If the current value is "));
                assert!(lines[cur_line+1].starts_with("    - Write the value "));
                let old_value = lines[cur_line]
                    .rsplit_once(' ').unwrap()
                    .1 == "1:";
                let write_value = lines[cur_line+1]
                    .rsplit_once(' ').unwrap()
                    .1 == "1.";
                let move_dir = match lines[cur_line+2]
                    .rsplit_once(' ').unwrap()
                    .1 {
                        "right." => Direction::Right,
                        "left." => Direction::Left,
                        _ => unreachable!("invalid direction")
                    };
                let next_state = lines[cur_line+3]
                    .rsplit_once(' ').unwrap()
                    .1.chars().next().unwrap();

                let rule_key = (state, old_value);
                let rule_action = (write_value, move_dir, next_state) as Action;
                rules.insert(rule_key, rule_action);
                cur_line += 4;
            }
            cur_line += 1;
        }

        TuringMachine {current_step, diagnostic_after, current_state, tape, tape_pos, rules}
    }
}
impl TuringMachine {
    fn diagnostic_checksum(&self) -> u32 {
        self.tape.values().filter(|&&x| x).count() as u32
    }
    fn next(&mut self) {
        let rules_key = (self.current_state, *self.tape.entry(self.tape_pos).or_insert(false));
        let (write_value, move_dir, next_state) = self.rules[&rules_key];
        self.tape.insert(self.tape_pos, write_value);
        match move_dir {
            Direction::Left => self.tape_pos -= 1,
            Direction::Right => self.tape_pos += 1,
        }
        self.current_state = next_state;
        self.current_step += 1;
    }
    fn exec(&mut self) {
        while self.current_step < self.diagnostic_after {
            self.next();
        }
    }
}

fn solve_part1(input_text: String) -> String {
    let mut tm = TuringMachine::from(input_text);
    // println!("{:?}", tm);
    tm.exec();
    tm.diagnostic_checksum().to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 25)
        .expect("Could not fetch input");

    println!("Example: {}", solve_part1(example_text()));
    println!("Part 1: {}", solve_part1(input_text));
}

fn example_text() -> String {
    String::from("\
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "3");
    }
}
