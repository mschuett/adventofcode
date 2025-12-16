use crate::helper;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug)]
struct Device {
    pub name: String,
    pub links: Vec<&'static Box<Self>>,
}


struct DeviceManager {
    devices: HashMap<String, Vec<String>>,
}
impl DeviceManager {
    fn from(input_text: &str) -> Self {
        let mut stringmap: HashMap<String, Vec<String>> = HashMap::new();
        let mut devices: Vec<Box<Device>> = Vec::with_capacity(500);
        let mut indexmap: HashMap<String, usize> = HashMap::new();
        let mut i: usize = 0;

        for line in input_text.lines() {
            let (cur, outstext) = line.split_once(": ").unwrap();
            let outs: Vec<String> = outstext.split_whitespace()
                .map(|s| s.to_string()).collect();
            stringmap.insert(cur.to_string(), outs.clone());
            let dev = Device { name: cur.to_string(), links: Vec::new() };
            let boxed = Box::new(dev);
            devices.push(boxed);
            indexmap.insert(cur.to_string(), i);
            i += 1;
        }

        // 2nd loop to fill in the Device links
        for (name, index) in indexmap.iter() {
            let dev_box_ref = &devices[*index];
            let dev = dev_box_ref.as_ref();

            let link_targets_strings = stringmap.get(name).unwrap();
            let mut link_targets_devs: Vec<&Box<Device>> = Vec::new();

            for target_string in link_targets_strings {
                let target_index = indexmap.get(target_string).unwrap();
                let target_dev = &devices[*target_index];
                link_targets_devs.push(&target_dev.to_owned());
            }

            let foo = dev;
            let bar = dev.links.to_owned();
            dev.links = link_targets_devs.to_owned();


        }
        DeviceManager { devices: stringmap }
    }
}

// simple recursive DFS
fn find_paths1(dm: &DeviceManager, current: &str) -> u32 {
    if current == "out" {
        return 1;
    }
    let next_devs = dm.devices.get(current).unwrap();
    let mut sum_of_nexts: u32 = 0;
    for next in next_devs {
        sum_of_nexts += find_paths1(dm, next);
    }
    sum_of_nexts
}

fn solve_part1(input_text: String) -> String {
    let dm = DeviceManager::from(&input_text);
    let result = find_paths1(&dm, "you");
    result.to_string()
}

// simple recursive DFS, but with target and terminal condition at 'out'
fn find_paths2(dm: &DeviceManager, current: &str, target: &str, fail_on: &Vec<&str>) -> u32 {
    if current == target {
        // println!("found path: {:?} {}", history, target);
        return 1;
    } else if fail_on.contains(&current) {
        return 0;
    }
    let next_devs = dm.devices.get(current).unwrap();
    let mut sum_of_nexts: u32 = 0;
    for next in next_devs {
        sum_of_nexts += find_paths2(dm, next, target, fail_on);
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

    let to_fft = check1
        .iter()
        .map(|c| find_paths2(&dm, "svr", c, &check2) * find_paths2(&dm, c, "fft", &check2))
        .sum::<u32>();
    println!("to_fft: {}", to_fft);

    let mut fft_dac = 0u32;
    for c2 in &check2 {
        for c3 in &check3 {
            for c4 in &check4 {
                // get paths from fft via c2,c3,c4 to dac
                let path = find_paths2(&dm, "fft", c2, &check3)
                    * find_paths2(&dm, c2, c3, &check4)
                    * find_paths2(&dm, c3, c4, &check5)
                    * find_paths2(&dm, c4, "dac", &check5);
                fft_dac += path;
                println!("fft-{}-{}-{}-dac: {}", c2, c3, c4, path)
            }
        }
    }
    println!("fft_dac: {}", fft_dac);

    let dac_out = check5
        .iter()
        .map(|c| find_paths2(&dm, "dac", c, &check5) * find_paths2(&dm, c, "out", &Vec::new()))
        .sum::<u32>();
    println!("dac_out: {}", dac_out);

    (to_fft as u64 * fft_dac as u64 * dac_out as u64).to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2025, 11).expect("Could not fetch input");

    let _ = example_text();
    let ts_start1 = Instant::now();
    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("took {} µs", ts_start1.elapsed().as_micros());

    let _ = example_text2();
    let ts_start2 = Instant::now();
    println!("Part 2: {}", solve_part2(input_text));
    println!("took {} ms", ts_start2.elapsed().as_millis());
}

fn example_text() -> String {
    String::from(
        "\
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
",
    )
}

fn example_text2() -> String {
    String::from(
        "\
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
",
    )
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
