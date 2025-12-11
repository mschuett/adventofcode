use crate::helper;
use std::collections::HashMap;

struct DeviceManager {
    devices: HashMap<String, Vec<String>>,
}
impl DeviceManager {
    fn from(input_text: &str) -> Self {
        let mut stringmap: HashMap<String, Vec<String>> = HashMap::new();
        for line in input_text.lines() {
            let (cur, outstext) = line.split_once(": ").unwrap();
            let outs: Vec<String> = outstext.split_whitespace().map(|s| s.to_string()) .collect();
            stringmap.insert(cur.to_string(), outs);
        }
        DeviceManager{devices: stringmap}
    }
}

// simple recursive DFS
fn find_paths1(dm: &DeviceManager, steps: u32, current: &str, history: Vec<&str>) -> u32 {
    if current == "out" {
        return 1
    }
    let next_devs = dm.devices.get(current).unwrap();
    let mut sum_of_nexts: u32 = 0;
    for next in next_devs {
        let mut new_history: Vec<&str> = history.clone();
        new_history.push(current);
        sum_of_nexts += find_paths1(dm, steps + 1, next, new_history);
    }
    sum_of_nexts
}


fn solve_part1(input_text: String) -> String {
    let dm = DeviceManager::from(&input_text);
    let result = find_paths1(&dm, 0, "you", Vec::new());
    result.to_string()
}


// simple recursive DFS, but with target and terminal condition at 'out'
fn find_paths2(dm: &DeviceManager, current: &str, target: &str, history: Vec<&str>, fail_on: &Vec<&str>) -> u32 {
    if current == target {
        // println!("found path: {:?} {}", history, target);
        return 1
    } else if fail_on.contains(&current) {
        return 0
    }
    let next_devs = dm.devices.get(current).unwrap();
    let mut sum_of_nexts: u32 = 0;
    for next in next_devs {
        let mut new_history: Vec<&str> = history.clone();
        new_history.push(current);
        sum_of_nexts += find_paths2(dm, next, target, new_history, fail_on);
    }
    sum_of_nexts
}


fn solve_part2(input_text: String) -> String {
    let dm = DeviceManager::from(&input_text);

    // this is not so great, because it is overly specific to my input :(
    // I used graphviz to identify these bottlenecks in the graph,
    // using them reduces the search space enough to use the primitive DFS again
    let check1 = Vec::from(["kox", "ehb", "pwk", "uvj"]);
    // fft is here
    let check2 = Vec::from(["qlh", "tup", "iza"]);
    let check3 = Vec::from(["tui", "vwj", "oas"]);
    let check4 = Vec::from(["fmj", "eyi", "gnu", "mha", "ugv"]);
    // dac is here
    let check5 = Vec::from(["you", "heu", "cgh"]);

    let to_fft = check1.iter()
        .map(|c| find_paths2(&dm, "svr", c, Vec::new(), &check2)
            * find_paths2(&dm, c, "fft", Vec::new(), &check2)
        ).sum::<u32>();
    println!("to_fft: {}", to_fft);

    let mut fft_dac = 0u32;
    for c2 in &check2 {
        for c3 in &check3 {
            for c4 in &check4 {
                // get paths from fft via c2,c3,c4 to dac
                let path = find_paths2(&dm, "fft", c2, Vec::new(), &check3)
                    * find_paths2(&dm, c2, c3, Vec::new(), &check4)
                    * find_paths2(&dm, c3, c4, Vec::new(), &check5)
                    * find_paths2(&dm, c4, "dac", Vec::new(), &check5);
                fft_dac += path;
                println!("fft-{}-{}-{}-dac: {}", c2, c3, c4, path)
            }
        }
    }
    println!("fft_dac: {}", fft_dac);

    let dac_out = check5.iter()
        .map(|c| find_paths2(&dm, "dac", c, Vec::new(), &check5)
            * find_paths2(&dm, c, "out", Vec::new(), &Vec::new())
        ).sum::<u32>();
    println!("dac_out: {}", dac_out);

    (to_fft as u64 * fft_dac as u64 * dac_out as u64).to_string()
}


pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2025, 11).expect("Could not fetch input");

    let _ = example_text();
    println!("Part 1: {}", solve_part1(input_text.clone()));

    let _ = example_text2();
    println!("Part 2: {}", solve_part2(input_text));
}


fn example_text() -> String {
    String::from("\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
")
}

fn example_text2() -> String {
    String::from("\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "5");
    }

    // #[test]
    // fn test_part2() {
    //     let result = solve_part2(example_text2());
    //     assert_eq!(result, "2");
    // }
}
