use std::collections::VecDeque;
use crate::helper;

type Component = (u16, u16);

#[derive(Clone, Debug)]
struct ComponentPool {
    parts: Vec<Component>,
}
impl From<String> for ComponentPool {
    fn from(input: String) -> Self {
        let parts = input.lines().map(|line| {
            let (a, b) = line.split_once('/').unwrap();
            (a.parse().unwrap(), b.parse().unwrap()) as Component
        }).collect();
        ComponentPool {parts}
    }
}
impl ComponentPool {
    fn options_for_port(&mut self, port: u16) -> Vec<Component> {
        // return matching parts
        self.parts.iter()
            .filter(|(a,b)| port == *a || port == *b )
            .copied()
            .collect::<Vec<_>>()
    }
    fn other_port(part: Component, used_port: u16) -> u16 {
        if part.0 == used_port {
            part.1
        } else {
            part.0
        }
    }
    fn score_bridge(parts: &[Component]) -> u16 {
        parts.iter().map(|(a,b)| a+b).sum::<u16>()
    }
}

#[derive(Clone, Debug)]
struct SearchState {
    pool: ComponentPool,
    path: Vec<Component>,
    last_port: u16,
    score: u16,
}

fn state_search(pool: ComponentPool, by_score: bool) -> SearchState {
    // part1: by_score=true, part2: by_score=false to find longest
    let start = SearchState {pool, path: Vec::new(), last_port: 0, score: 0 };
    let mut best_state: SearchState = start.clone();
    let mut queue: VecDeque<SearchState> = VecDeque::new();
    queue.push_back(start);

    while let Some(mut state) = queue.pop_front() {
        let next_options = state.pool.options_for_port(state.last_port);
        if next_options.is_empty() {  // no more steps to go
            if by_score {
                if state.score > best_state.score {
                    best_state = state;
                    continue;
                }
            } else if state.path.len() > best_state.path.len()
                || (state.path.len() == best_state.path.len() && state.score > best_state.score) {
                best_state = state;
                continue;
            }
        }
        for option in next_options {
            let mut new_state = state.clone();
            new_state.last_port = ComponentPool::other_port(option, state.last_port);
            new_state.score += option.0 + option.1;
            new_state.path.push(option);
            new_state.pool.parts.remove(
                new_state.pool.parts.iter().position(|&p| p == option).unwrap()
            );
            queue.push_front(new_state);  // DFS
        }
    }
    best_state
}

fn solve_part1(input_text: String) -> String {
    let pool = ComponentPool::from(input_text);
    let best = state_search(pool, true);
    println!("found {:?} with length {} and score {}", best.path, best.path.len(), best.score);
    best.score.to_string()
}

fn solve_part2(input_text: String) -> String {
    let pool = ComponentPool::from(input_text);
    let best = state_search(pool, false);
    println!("found {:?} with length {} and score {}", best.path, best.path.len(), best.score);
    best.score.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 24)
        .expect("Could not fetch input");

    println!("Example1: {}", solve_part1(example_text()));
    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Example2: {}", solve_part2(example_text()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "31");
    }

    #[test]
    fn test_score() {
        let parts: Vec<Component> = vec![(0, 1), (10, 1), (9, 10)];
        let score = ComponentPool::score_bridge(&parts);
        assert_eq!(score, 31);
    }

    #[test]
    fn test_pop_options() {
        let mut pool = ComponentPool::from(example_text());
        assert_eq!(pool.parts, vec![(0, 2), (2, 2), (2, 3), (3, 4), (3, 5), (0, 1), (10, 1), (9, 10)]);
        let options = pool.options_for_port(0);
        assert_eq!(options, vec![(0, 2), (0, 1)]);
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "19");
    }
}
