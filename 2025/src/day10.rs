use crate::helper;
use std::fmt;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

struct Machine {
    desired_state: Vec<bool>,
    buttons: Vec<Vec<u8>>,
    joltages: Vec<u16>
}

// for parsing
impl Machine {
    pub fn from(line: &str) -> Machine {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let state_field = fields[0];
        assert_eq!(state_field.chars().next().unwrap(), '[');
        let desired_state: Vec<bool> = state_field
            .trim_prefix('[')
            .trim_suffix(']')
            .chars()
            .map(|c| c == '#')
            .collect();
        let joltage_field = fields[fields.len()-1];
        assert_eq!(joltage_field.chars().next().unwrap(), '{');
        let joltages: Vec<u16> = joltage_field
            .trim_prefix('{')
            .trim_suffix('}')
            .split(',')
            .map(|s| s.parse::<u16>().unwrap())
            .collect();
        let mut buttons: Vec<Vec<u8>> = Vec::new();
        for field in &fields[1..fields.len()-1] {
            assert_eq!(field.chars().next().unwrap(), '(');
            let button_label: Vec<u8> = field
                .trim_prefix('(')
                .trim_suffix(')')
                .split(",")
                .map(|s| s.parse::<u8>().unwrap())
                .collect();
            buttons.push(button_label);
        }
        Machine{desired_state, buttons, joltages}
    }
}
impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.desired_state, self.buttons, self.joltages)
    }
}

fn parse_input(input_text: &str) -> Vec<Machine> {
    input_text
        .lines()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(Machine::from)
        .collect::<Vec<Machine>>()
}

// to search the state space
#[derive(Debug)]
struct ButtonPathState {
    step: u32,
    state: Vec<bool>,
    state_history: Vec<Vec<bool>>,
    button_history: Vec<Vec<u8>>,
}
impl ButtonPathState {
    fn from(machine: &Machine) -> ButtonPathState {
        let state_len = machine.desired_state.len();
        ButtonPathState{step: 0, state: vec![false; state_len], state_history: vec![], button_history: vec![]}
    }
    fn next_steps(&self, machine: &Machine) -> VecDeque<ButtonPathState> {
        let mut next: VecDeque<ButtonPathState> = VecDeque::with_capacity(machine.buttons.len());
        for button in &machine.buttons {
            // gen new state
            let mut new_state: Vec<bool> = self.state.clone();
            for i in button {
                new_state[*i as usize] = !new_state[*i as usize];
            }
            // do not loop back to previous states
            if self.state_history.contains(&new_state) { continue }

            // gen and push new pathState object
            let mut new_state_history = self.state_history.clone();
            new_state_history.push(new_state.clone());
            let mut new_button_history = self.button_history.clone();
            new_button_history.push(button.clone());
            let new_bps = ButtonPathState{
                step: self.step + 1,
                state: new_state,
                state_history: new_state_history,
                button_history: new_button_history,
            };
            next.push_back(new_bps);
        }
        next
    }
    // recursive, breadth-first search
    fn search_button_path(machine: &Machine, shortest_map: &mut HashMap<Vec<bool>, u32>, states: VecDeque<ButtonPathState>) -> ButtonPathState {
        let mut new_states: VecDeque<ButtonPathState> = VecDeque::new();
        for bps in states {
            if bps.state == machine.desired_state {
                return bps
            }
            let next_steps = bps.next_steps(machine);
            for next_step in next_steps {
                if shortest_map.contains_key(&next_step.state) {
                    let old_steps = shortest_map.get(&next_step.state).unwrap();
                    if old_steps <= &next_step.step {
                        // new path is longer => do not follow
                        continue
                    }
                }
                // new state, or new path is shorter => update map and follow path
                shortest_map.insert(next_step.state.clone(), next_step.step);
                new_states.push_back(next_step);
            }
        }
        ButtonPathState::search_button_path(machine, shortest_map, new_states)
    }
}

fn solve_part1(input_text: String) -> String {
    let machines = parse_input(&input_text);
    let mut solution_steps: Vec<u32> = Vec::with_capacity(machines.len());
    for m in machines {
        // println!("{}", m);
        let state = ButtonPathState::from(&m);
        // shortest map keeps track of the lowest number of steps to reach a state
        let mut shortest_map: HashMap<Vec<bool>, u32> = HashMap::new();
        shortest_map.insert(state.state.clone(), state.step);
        let mut deq = VecDeque::from([state]);

        let solution = ButtonPathState::search_button_path(&m, &mut shortest_map, deq);
        // println!("=> {} buttons: {:?}", solution.step, solution.button_history);
        solution_steps.push(solution.step);
    }

    solution_steps.iter().sum::<u32>().to_string()
}

// using an LP solver (microlp)
// this is a way too comlicated function to generate its input config
fn lp_solve(joltages: Vec<u16>, buttons: Vec<Vec<u8>>, machine_number: usize) -> u32 {
    let upper_bound: i32 = *joltages.iter().max().unwrap() as i32;
    let num_vars = buttons.len();
    let mut problem = microlp::Problem::new(microlp::OptimizationDirection::Minimize);

    let mut vars: Vec<microlp::Variable> = Vec::with_capacity(num_vars);
    for _i in 0..num_vars {
        // every button gets a variable
        let v = problem.add_integer_var(1.0, (0, upper_bound));
        vars.push(v);
    }

    // also write an lp_solve text for verification/debugging
    let mut text: String = String::from("");
    let text_vars: Vec<String> = (0..num_vars)
        .map(|i| ((b'a' + i as u8) as char).to_string())
        .collect();
    text += &*("min: ".to_owned() + &text_vars.join(" + ") + ";\n\n");
    text += &text_vars.iter().map(|c| format!("{} >= 0;\n", c)).collect::<String>();

    for (j, joltage) in joltages.iter().enumerate() {
        // every joltage gets a constraint
        let result = *joltage as f64;
        let mut lin_exp: microlp::LinearExpr = microlp::LinearExpr::empty();

        let mut text_constraint_vars: Vec<String> = Vec::with_capacity(num_vars);
        // get all buttons that increase this joltage and add to expression
        for i in 0..num_vars {
            if buttons[i].contains(&(j as u8)) {
                lin_exp.add(vars[i], 1.0);
                text_constraint_vars.push(text_vars[i].clone());
            }
        }
        text += format!("{} = {};\n", text_constraint_vars.join("+"), joltage).as_str();
        problem.add_constraint(lin_exp, microlp::ComparisonOp::Eq, result);
    }

    // finish debug output to lpsolve.txt file
    text += format!("\nint var {};\n", &text_vars.join(", ")).as_str();
    let mut file = File::create(format!("day10_{}_lpsolve.txt", machine_number)).unwrap();
    file.write_all(text.as_bytes()).unwrap();

    // argh, this is so stupid. the round() is important, without it my answer is 'too low' :-(
    let solution = problem.solve().unwrap();
    solution.objective().round() as u32
}


fn solve_part2(input_text: String) -> String {
    let machines = parse_input(&input_text);
    let mut solution_steps: Vec<u32> = Vec::with_capacity(machines.len());
    for (i, m) in machines.into_iter().enumerate() {
        let solution = lp_solve(m.joltages, m.buttons, i);
        // println!("#{} => {} buttons", i, solution);
        solution_steps.push(solution);
    }
    let sum = solution_steps.iter().sum::<u32>();
    // println!("{} machines => sum {}", solution_steps.len(), sum);
    sum.to_string()
}


pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2025, 10).expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}


fn example_text() -> String {
    String::from("\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "7");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "33");
    }
}
