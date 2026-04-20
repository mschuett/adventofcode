use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use crate::helper;

fn parse_input(s: String) -> HashMap<u16, Vec<u16>> {
    let mut map: HashMap<u16, Vec<u16>> = HashMap::new();
    s.lines().for_each(|line| {
        let (lhs, rhs) = line.split_once(" <-> ").unwrap();
        let lhs_num: u16 = lhs.parse().unwrap();
        let rhs_nums: Vec<u16> = rhs.split_whitespace().map(|word| word.trim_end_matches(",").parse::<u16>().unwrap()).collect::<Vec<u16>>();
        map.insert(lhs_num, rhs_nums);
    });
    map
}

fn find_group_size(map: &HashMap<u16, Vec<u16>>, element: u16) -> usize {
    let mut todo: VecDeque<u16> = VecDeque::from(vec![element]);
    let mut set: HashSet<u16> = HashSet::new();
    while !todo.is_empty() {
        let item = todo.pop_front().unwrap();
        set.insert(item);
        for connected in map.get(&item).unwrap() {
            if !set.contains(connected) {
                set.insert(*connected);
                todo.push_back(*connected);
            }
        }
    }
    set.len()
}

fn solve_part1(input_text: String) -> String {
    let map = parse_input(input_text);
    find_group_size(&map, 0).to_string()
}

pub fn find_group_count(map: &HashMap<u16, Vec<u16>>) -> usize {
    // approach: every group reveives an arbitrary group id
    let mut groups: HashMap<u16, u16> = HashMap::new();
    let mut new_id: u16 = 0;
    for num in map.keys() {
        let mut items = map.get(num).unwrap().clone();
        items.push(*num);
        // find all existing group ids
        let ids: Vec<u16> = items.iter()
            .filter_map(|num| groups.get(num))
            .copied()
            .unique()
            .collect();
        // three cases: no/one/multiple ids
        if ids.is_empty() {
            // give new id
            for item in items {
                groups.insert(item, new_id);
            }
            new_id += 1;
        } else if ids.len() == 1 {
            // found id, (re-)apply to all items
            let id = ids[0];
            for item in items {
                groups.insert(item, id);
            }
        } else if ids.len() > 1 {
            // multiple ids => re-label all numbers of these groups
            let id = ids[0];
            // update all prevous mapped values
            groups.iter_mut().for_each(|(_, map_id)|
              if ids.contains(map_id) { *map_id = id }
            );
            // also update/set the current items
            for item in items {
                groups.insert(item, id);
            }
        } else {
            panic!("should not happen");
        }
    }
    groups.values().unique().count()
}

fn solve_part2(input_text: String) -> String {
    let map = parse_input(input_text);
    find_group_count(&map).to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 12)
            .expect("Could not fetch input");

    println!("Example: {}", solve_part1(example_text()));
    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "6");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "2");
    }
}
