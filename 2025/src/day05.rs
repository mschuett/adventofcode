use crate::helper;

fn solve_part1(input_text: String) -> String {
    let (ranges_block, available_block) = input_text.trim().split_once("\n\n").unwrap();
    let mut ranges = ranges_block
        .lines()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| line.split_once("-").unwrap())
        .collect::<Vec<(&str, &str)>>()
        .into_iter()
        .map(|(a,b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect::<Vec<(u64, u64)>>();
    ranges.sort_by(|a, b| a.1.cmp(&b.1));

    let ingredients = available_block
        .lines()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut fresh_count = 0;
    for item in &ingredients {
        for (from,to) in &ranges {
            if from <= item && item <= to {
                fresh_count += 1;
                // println!("{item} is in {from}-{to} -> count {fresh_count}");
                break
            }
        }
    }

    fresh_count.to_string()
}

fn solve_part2(input_text: String) -> String {
    let (ranges_block, _) = input_text.trim().split_once("\n\n").unwrap();
    let mut ranges = ranges_block
        .lines()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| line.split_once("-").unwrap())
        .collect::<Vec<(&str, &str)>>()
        .into_iter()
        .map(|(a,b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect::<Vec<(u64, u64)>>();
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    // merge ranges, not very efficient
    let mut changed_sth = true;
    while changed_sth {
        changed_sth = false;
        'i: for i in 0..ranges.len() {
            for j in i + 1..ranges.len() {
                // readable var names
                let (i_from, i_to) = ranges[i];
                let (j_from, j_to) = ranges[j];
                // fully contained => useless
                // jjjjjjjj
                //   iii
                if i_from >= j_from && i_to <= j_to {
                    ranges.remove(i);
                    changed_sth = true;
                    break 'i  // because we changed the loop index
                }
                //   jjjj
                // iiiiiiii
                if i_from <= j_from && i_to >= j_to {
                    ranges.remove(j);
                    changed_sth = true;
                    break 'i  // because we changed the loop index
                }
                // partial overlap => merge
                //   jjjjjjj
                //      iiiiiii
                if i_from >= j_from && i_to >= j_to && i_from <= j_to {
                    ranges[j].1 = i_to;
                    ranges.remove(i);
                    changed_sth = true;
                    break 'i  // because we changed the loop index
                }
                // partial overlap => merge
                //    jjjjjjj
                // iiiiiii
                if j_from >= i_from && j_to >= i_to && j_from <= i_to {
                    ranges[i].1 = j_to;
                    ranges.remove(j);
                    changed_sth = true;
                    break 'i  // because we changed the loop index
                }
            }
        }
    }
    let mut count = 0;
    for (from,to) in ranges {
        let range_count = to-from+1;
        // println!("{from}-{to}, count {range_count}");
        count += range_count;
    }
    count.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2025, 5)
            .expect("Could not fetch input");
    let _ = example_text();

    print!("Part 1: {}\nPart 2: {}\n",
           solve_part1(input_text.clone()),
           solve_part2(input_text));
}

fn example_text() -> String {
    String::from(
        "3-5
10-14
16-20
12-18

1
5
8
11
17
32
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

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "14");
    }
}

