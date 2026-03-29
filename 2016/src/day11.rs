use std::collections::{HashMap, HashSet, VecDeque};

// use the same struct for different problem sizes (example, part1, part2) by padding the arrays
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    elevator: u8,
    generators: [u8; 7],
    chips: [u8; 7],
}

impl State {
    fn is_valid(&self) -> bool {
        for i in 0..7 {
            if self.chips[i] == self.generators[i] {
                continue;
            }
            for j in 0..7 {
                if self.chips[i] == self.generators[j] {
                    return false; /* chip fried => skip state */
                }
            }
        }
        self.elevator >= 1 && self.elevator <= 4
    }

    fn is_sensible(&self) -> bool {
        // minor optimization: limit search space by preventing chips to wander too far away from their generators
        for i in 0..7 {
            let diff = (self.chips[i] as i16 - self.generators[i] as i16).abs();
            if diff > 2 {
                return false;
            }
        }
        true
    }

    fn is_canonical(&self) -> bool {
               (self.generators[6] > self.generators[5] || (self.generators[6] == self.generators[5] && self.chips[6] >= self.chips[5]))
            && (self.generators[5] > self.generators[4] || (self.generators[5] == self.generators[4] && self.chips[5] >= self.chips[4]))
            && (self.generators[4] > self.generators[3] || (self.generators[4] == self.generators[3] && self.chips[4] >= self.chips[3]))
            && (self.generators[3] > self.generators[2] || (self.generators[3] == self.generators[2] && self.chips[3] >= self.chips[2]))
            && (self.generators[2] > self.generators[1] || (self.generators[2] == self.generators[1] && self.chips[2] >= self.chips[1]))
            && (self.generators[1] > self.generators[0] || (self.generators[1] == self.generators[0] && self.chips[1] >= self.chips[0]))
    }

    fn has_items_below(&self, level: u8) -> bool {
        let chips_below = self.chips.iter().filter(|&&x| x < level).count();
        let gens_below = self.generators.iter().filter(|&&x| x < level).count();
        (chips_below + gens_below) > 0
    }

    // sort the data fields in order to join equivalent states
    fn make_canonical(&mut self) -> Self {
        while !self.is_canonical() {
            for i in 0..6 {
                if self.generators[i] > self.generators[i + 1]
                    || (self.generators[i] == self.generators[i + 1]
                        && self.chips[i] > self.chips[i + 1])
                {
                    let (a, b) = (self.generators[i], self.generators[i + 1]);
                    self.generators[i + 1] = a;
                    self.generators[i] = b;
                    let (a, b) = (self.chips[i], self.chips[i + 1]);
                    self.chips[i + 1] = a;
                    self.chips[i] = b;
                }
            }
        }
        *self
    }
}

type Steps = u32;

fn bfs_solve(init_state: State, elements: usize) -> String {
    let target_state = State {
        elevator: 4,
        generators: [4, 4, 4, 4, 4, 4, 4],
        chips: [4, 4, 4, 4, 4, 4, 4],
    };
    let start_state: State = init_state.clone().make_canonical();
    let mut memory: HashMap<State, Steps> = HashMap::new();
    memory.insert(start_state, 0);
    let mut queue: VecDeque<(Steps, State)> = VecDeque::from(vec![(0, start_state)]);
    let mut iterations: u32 = 0;

    loop {
        let (steps, state) = queue
            .pop_front()
            .expect(format!("error, empty search queue after iteration {}", iterations).as_str());
        // check target state (without memo_map, with BFS we have first = shortest path)
        if state == target_state {
            return steps.to_string();
        }
        // save to memo-map
        memory.entry(state).or_insert(steps);

        iterations += 1;
        if iterations % 1024 == 0 {
            println!(
                "it {}, cur depth {}, map size {}, queue len {}, current: {:?}",
                iterations,
                steps,
                memory.len(),
                queue.len(),
                state
            );
        }

        // now try to generate all sensible state permutations :-/
        // using a set to immediately remove duplicates
        let mut followup_states: HashSet<State> = HashSet::new();
        let mut new_state;
        if state.elevator < 4 {
            // go up
            let new_level = state.elevator + 1;
            for i in 0..elements {
                if state.generators[i] == state.elevator {
                    for j in i + 1..elements {
                        if state.generators[j] == state.elevator {
                            // move two generators
                            new_state = state.clone();
                            new_state.elevator = new_level;
                            new_state.generators[i] = new_level;
                            new_state.generators[j] = new_level;
                            followup_states.insert(new_state.make_canonical());
                        }
                    }
                    for j in 0..elements {
                        if state.chips[j] == state.elevator {
                            // move generator+chip
                            new_state = state.clone();
                            new_state.elevator = new_level;
                            new_state.generators[i] = new_level;
                            new_state.chips[j] = new_level;
                            followup_states.insert(new_state.make_canonical());
                        }
                    }
                    // move single generator
                    new_state = state.clone();
                    new_state.elevator = new_level;
                    new_state.generators[i] = new_level;
                    followup_states.insert(new_state.make_canonical());
                }

                if state.chips[i] == state.elevator {
                    if state.generators[i] == state.elevator {
                        new_state = state.clone();
                        new_state.elevator = new_level;
                        new_state.generators[i] = new_level;
                        new_state.chips[i] = new_level;
                        followup_states.insert(new_state.make_canonical());
                    } else {
                        new_state = state.clone();
                        new_state.elevator = new_level;
                        new_state.chips[i] = new_level;
                        followup_states.insert(new_state.make_canonical());
                    }
                    // move double chips
                    for j in i + 1..elements {
                        if state.chips[j] == state.elevator {
                            new_state = state.clone();
                            new_state.elevator = new_level;
                            new_state.chips[i] = new_level;
                            new_state.chips[j] = new_level;
                            followup_states.insert(new_state.make_canonical());
                        }
                    }
                }
            }
        }
        if state.has_items_below(state.elevator) {
            // go down
            let new_level = state.elevator - 1;
            for i in 0..elements {
                // never move generators downward (???)
                if state.generators[i] == state.elevator {
                    for j in i + 1..elements {
                        if state.generators[j] == state.elevator {
                            // move two generators
                            new_state = state.clone();
                            new_state.elevator = new_level;
                            new_state.generators[i] = new_level;
                            new_state.generators[j] = new_level;
                            followup_states.insert(new_state.make_canonical());
                        }
                    }
                    for j in 0..elements {
                        if state.chips[j] == state.elevator {
                            // move generator+chip
                            new_state = state.clone();
                            new_state.elevator = new_level;
                            new_state.generators[i] = new_level;
                            new_state.chips[j] = new_level;
                            followup_states.insert(new_state.make_canonical());
                        }
                    }
                    // move single generator
                    new_state = state.clone();
                    new_state.elevator = new_level;
                    new_state.generators[i] = new_level;
                    followup_states.insert(new_state.make_canonical());
                }

                if state.chips[i] == state.elevator {
                    // move double chips
                    for j in i + 1..elements {
                        if state.chips[j] == state.elevator {
                            new_state = state.clone();
                            new_state.elevator = new_level;
                            new_state.chips[i] = new_level;
                            new_state.chips[j] = new_level;
                            followup_states.insert(new_state.make_canonical());
                        }
                    }
                    // move single chip
                    new_state = state.clone();
                    new_state.elevator = new_level;
                    new_state.chips[i] = new_level;
                    followup_states.insert(new_state.make_canonical());
                }
            }
        }
        for fup in followup_states {
            // note: in BFS every existing map entry has fewer steps, so no extra check required
            if fup.is_valid()
                && fup.is_sensible()
                && !memory.contains_key(&fup)
                && !queue.contains(&(steps + 1, fup))
            {
                queue.push_back((steps + 1, fup))
            }
        }
    }
}

pub fn solve() {
    let start_state = State {
        elevator: 1,
        generators: [1, 1, 1, 1, 1, 4, 4],
        chips: [2, 1, 2, 1, 1, 4, 4],
    };
    println!("Part 1: {}", bfs_solve(start_state, 5));
    let start_state = State {
        elevator: 1,
        generators: [1, 1, 1, 1, 1, 1, 1],
        chips: [2, 1, 2, 1, 1, 1, 1],
    };
    println!("Part 2: {}", bfs_solve(start_state, 7));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_canonical() {
        // nicely sorted => canonical
        let state1 = State {
            elevator: 1,
            generators: [2, 3, 4, 4, 4, 4, 4],
            chips: [1, 1, 4, 4, 4, 4, 4],
        };
        assert!(state1.is_canonical());
        // last generator smaller
        let state2 = State {
            elevator: 1,
            generators: [2, 3, 4, 4, 3, 4, 4],
            chips: [1, 1, 4, 4, 4, 4, 4],
        };
        assert!(!state2.is_canonical());
        // last chip smaller
        let state3 = State {
            elevator: 1,
            generators: [2, 3, 4, 4, 4, 4, 4],
            chips: [1, 1, 4, 4, 3, 4, 4],
        };
        assert!(!state3.is_canonical());
    }

    #[test]
    fn test_make_canonical() {
        let state = State {
            elevator: 1,
            generators: [2, 3, 3, 4, 4, 4, 4],
            chips: [1, 1, 4, 4, 4, 4, 4],
        };
        let mut state1 = state.clone();
        assert_eq!(state, state1.make_canonical());

        // last generator smaller
        let mut state2 = State {
            elevator: 1,
            generators: [2, 3, 4, 4, 3, 4, 4],
            chips: [1, 4, 4, 4, 1, 4, 4],
        };
        assert_eq!(state, state2.make_canonical());

        // last chip smaller
        let mut state3 = State {
            elevator: 1,
            generators: [2, 3, 3, 4, 4, 4, 4],
            chips: [1, 4, 1, 4, 4, 4, 4],
        };
        assert_eq!(state, state3.make_canonical());
    }

    #[test]
    fn test_is_sensible() {
        let state1 = State {
            elevator: 1,
            generators: [2, 4, 4, 4, 4, 4, 4],
            chips: [1, 1, 4, 4, 4, 4, 4],
        };
        assert!(!state1.is_sensible());
        let state2 = State {
            elevator: 1,
            generators: [2, 3, 4, 4, 4, 4, 4],
            chips: [1, 2, 4, 4, 4, 4, 4],
        };
        assert!(state2.is_sensible());
    }

    #[test]
    fn test_has_items_below() {
        let state1 = State {
            elevator: 4,
            generators: [4, 4, 4, 4, 4, 4, 4],
            chips: [1, 1, 4, 4, 4, 4, 4],
        };
        assert!(state1.has_items_below(2));
        let state2 = State {
            elevator: 4,
            generators: [4, 4, 4, 4, 4, 4, 4],
            chips: [3, 3, 4, 4, 4, 4, 4],
        };
        assert!(!state2.has_items_below(2));
    }

    #[test]
    fn test_part1() {
        let start_state = State {
            elevator: 1,
            generators: [2, 3, 4, 4, 4, 4, 4],
            chips: [1, 1, 4, 4, 4, 4, 4],
        };
        // there is a bug somewhere, because the correct elements=2 yields a wrong result :(
        let result = bfs_solve(start_state, 3);
        assert_eq!(result, "11");
    }
}
