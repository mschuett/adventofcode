use std::collections::{HashMap, VecDeque};
use crate::helper;

#[derive(Clone,Copy,Debug,PartialEq)]
struct Bot {
    low: u8,
    high: u8,
}
impl Bot {
    fn receive(&mut self, val: u8) {
        if self.low == 0 {
            self.low = val;
            return;
        }
        if self.high == 0 {
            let minmax = (std::cmp::min(self.low, val), std::cmp::max(self.low, val));
            self.low = minmax.0;
            self.high = minmax.1;
            return;
        }
        unimplemented!("bot receives third value?")
    }
}

fn solve_both(input_text: String) -> String {
    let mut bots: HashMap<u8, Bot> = HashMap::new();
    let mut output: [u8; 21] = [0; 21];
    let mut commands: VecDeque<Vec<&str>> = VecDeque::new();
    let mut result_botnum: u8 = 0;

    // first iteration: distribute initial values to bots
    // and remember everything else as Command-Deque
    for line in input_text.lines() {
        let words = line.split(' ').collect::<Vec<&str>>();
        match words[0] {
            "value" => {
                assert_eq!(words.len(), 6);
                assert_eq!(words[2], "goes");
                assert_eq!(words[3], "to");
                assert_eq!(words[4], "bot");
                let val = words[1].parse::<u8>().unwrap();
                let bot = words[5].parse::<u8>().unwrap();
                bots
                    .entry(bot)
                    .or_insert(Bot { low: 0, high: 0 })
                    .receive(val);
            }
            "bot" => {
                assert_eq!(words.len(), 12);
                assert_eq!(words[0], "bot");
                assert_eq!(words[2], "gives");
                assert_eq!(words[3], "low");
                assert_eq!(words[4], "to");
                assert_eq!(words[7], "and");
                assert_eq!(words[8], "high");
                assert_eq!(words[9], "to");
                commands.push_back(words);
            }
            _ => unreachable!("invalid input")
        }
    }
    // println!("initial:");
    // for bot in bots.keys().sorted() {
    //     println!("{:?}", bots.get(bot).unwrap());
    // }

    // now process the commands
    while !commands.is_empty() {
        let words = commands.pop_front().unwrap();
        let botnum = words[1].parse::<u8>().unwrap();
        if !bots.contains_key(&botnum) {
            // bot not found, put cmd to back of list
            commands.push_back(words);
            continue;
        }
        let src_bot = bots.get(&botnum).unwrap();
        if src_bot.low == 0 || src_bot.high == 0 {
            // bot not ready, put cmd to back of list
            commands.push_back(words);
            continue;
        }

        // bot exists, and has two chips => remove and process it
        let (_, src_bot): (u8, Bot) = bots
            .remove_entry(&botnum).unwrap();
        let target1 = words[6].parse::<u8>().unwrap();
        if words[5] == "output" {
            output[target1 as usize] = src_bot.low;
        } else {
            bots.entry(target1).or_insert(Bot { low: 0, high: 0 });
            let target_bot = bots.get_mut(&target1).unwrap();
            target_bot.receive(src_bot.low);

            if target_bot.low == 17 && target_bot.high == 61 {
                result_botnum = target1;
            }
        }

        let target2 = words[11].parse::<u8>().unwrap();
        if words[10] == "output" {
            output[target2 as usize] = src_bot.high;
        } else {
            bots.entry(target2).or_insert(Bot { low: 0, high: 0 });
            let target_bot = bots.get_mut(&target2).unwrap();
            target_bot.receive(src_bot.high);
            if target_bot.low == 17 && target_bot.high == 61 {
                result_botnum = target2;
            }
        }
    }

    // println!("after shuffling:");
    // for bot in bots.values() {
    //     println!("{:?}", bot);
    // }
    // println!("output: {:?}", output);

    println!("Part 1: {}", result_botnum);
    println!("Part 2: {}", output[0] as u32 * output[1] as u32 * output[2] as u32);
    "".to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 10)
            .expect("Could not fetch input");
    let _ = example_text();

    solve_both(input_text);
}

fn example_text() -> String {
    String::from(
        "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2")
}
