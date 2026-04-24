use std::collections::{HashMap, VecDeque};
use crate::helper;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Program {
    name: String,
    weight: u32,
    children: Vec<Box<Program>>,
}
impl Program {
    fn from_words(name: String, weight_str: &str) -> Program {
        let weight = weight_str
            .strip_prefix("(").unwrap()
            .strip_suffix(")").unwrap()
            .parse::<u32>().unwrap();
        Program {name, weight, children: vec![]}
    }
    fn full_weight(&self) -> u32 {
        self.weight + self.children.iter().map(|c| c.full_weight()).sum::<u32>()
    }
    fn is_balanced(&self) -> bool {
        if self.children.is_empty() {
            return true;
        }
        let first = self.children.first().unwrap().full_weight();
        self.children[1..].iter().all(|child| child.full_weight() == first)
    }
    fn fix_wrong_weight(&self) -> u32 {
        for child in &self.children {
            if !child.is_balanced() {
                return child.fix_wrong_weight();
            }
        }
        // children are balanced, but self is not
        let child_weights: Vec<u32> = self.children.iter()
            .map(|child| child.full_weight())
            .collect();
        println!("unbalanced: {:?} with children weights {:?}", self.name, child_weights);
        // trust there is only one outlyer, so any two identical weights are the expected norm
        assert!(self.children.len() >= 3);
        let target_weight: u32;
        if child_weights[0] == child_weights[1] {
            target_weight = child_weights[1];
        } else {
            target_weight = child_weights[2];
        }
        let (deviating_index, correction) = child_weights.iter()
            .enumerate()
            .filter_map(|(i, &w)| {
                let diff: i32 = w as i32 - target_weight as i32;
                if diff != 0 {
                    Some((i, diff))
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, i32)>>()[0];
        let fixed = (self.children[deviating_index].weight as i32 + correction) as u32;
        println!("child #{} ({}) is wrong. has {}, should have {}",
                 deviating_index, self.children[deviating_index].name, self.children[deviating_index].weight, fixed);
        fixed
    }
}

fn parse_input(input: &str) -> Box<Program> {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l
            .split_whitespace()
            .map(|s| s
                    .strip_suffix(",")
                    .unwrap_or(s)
            ).collect::<Vec<&str>>()
        ).collect::<Vec<Vec<&str>>>();
    // first iteration: generate objects without links,
    // but already sort into leaves (done) and other tree nodes (with work to do)
    let mut leaves: HashMap<&str, Box<Program>> = HashMap::new();
    let mut upper_levels: HashMap<&str, Box<Program>> = HashMap::new();
    let mut lines_todo: VecDeque<&Vec<&str>> = VecDeque::new();
    for words in lines.iter() {
        let prog = Program::from_words(words[0].to_string(), words[1]);
        if words.len() == 2 {
            leaves.insert(words[0], Box::new(prog));
        } else {
            upper_levels.insert(words[0], Box::new(prog));
            lines_todo.push_back(words);
        }
    }
    // now try to iterate through the tree layers, every completed layer becomes the new 'leaves'
    while !lines_todo.is_empty() {
        let line = lines_todo.pop_front().unwrap();
        assert_eq!(line[2], "->");
        // check if all leaves are available
        let deps = line[3..].to_vec();
        if deps.iter().any(|d| !leaves.contains_key(d)) {  // missing leaf, wrong layer
            lines_todo.push_back(line);
            continue;
        }
        // else: on right layer => process this line,
        // i.e. move leaves into parent object, that becomes a new leaf
        let mut cur_prog = upper_levels.remove(line[0]).unwrap();
        for dep in deps {
            let dep_box = leaves.remove(dep).unwrap();
            cur_prog.children.push(dep_box);
        }
        leaves.insert(line[0], cur_prog);
    }
    // now we should have the single root program as the only leaf
    assert!(upper_levels.is_empty());
    assert_eq!(leaves.len(), 1);
    leaves.values().nth(0).unwrap().clone()
}

fn solve_part1(input_text: String) -> String {
    let root = parse_input(input_text.as_str());
    root.name
}

fn solve_part2(input_text: String) -> String {
    let root = parse_input(input_text.as_str());
    root.fix_wrong_weight().to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 7)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "tknk");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "60");
    }
}
