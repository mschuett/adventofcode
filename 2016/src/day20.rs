use std::cmp::{min, max, Ordering};
use std::collections::VecDeque;
use crate::helper;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Range {
    min: u32,
    max: u32,
}
impl Range {
    fn contains(&self, x: u32) -> bool {
        x >= self.min && x <= self.max
    }
    fn size(&self) -> u32 {
        self.max - self.min + 1
    }
}
impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.min
            .cmp(&other.min)
            .then(self.max.cmp(&other.max))
    }
}
impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Range {
    fn from_string(line: &str) -> Range {
        let nums = line
            .split("-")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        Range{min: nums[0], max: nums[1]}
    }
    fn merge(&self, other: &Range) -> Result<Range, String> {
        // merge if ranges are overlapping, otherwise fail

        // reduce number of cases by sorting:
        let low = *min(&self, &other);
        let high = *max(&self, &other);

        // check if one range is strict superset
        if low.min <= high.min && low.max >= high.max {
            return Ok(Range{min: low.min, max: low.max})
        }
        // overlapping ranges
        if low.min <= high.min && low.max >= high.min && low.max <= high.max {
            return Ok(Range{min: low.min, max: high.max})
        }
        // edge-case: non-overlapping, but adjacent ranges
        if low.max+1 == high.min {
            return Ok(Range{min: low.min, max: high.max})
        }

        // non-overlapping ranges
        if low.max < high.min-1 {
            return Err("non-overlapping ranges".to_string())
        }
        unimplemented!("missing case in merge()?");
    }
}

fn solve_part1(input_text: String) -> String {
    let mut ranges = input_text
        .lines()
        .map(|l| Range::from_string(l))
        .collect::<Vec<Range>>();
    ranges.sort();

    // reduce -- note: for part 1 we only have to continually merge the first range,
    // and the first non-mergable gap has the solution
    let mut first = ranges[0];
    for i in 1..ranges.len() {
        let result = first.merge(&ranges[i]);
        if result.is_ok() {
            first = result.unwrap();
        } else {
            return (first.max + 1).to_string();
        }
    }
    !unreachable!("everything merges, no non-blocked IP number")
}

fn solve_part2(input_text: String) -> String {
    let mut range_vec = input_text
        .lines()
        .map(|l| Range::from_string(l))
        .collect::<Vec<Range>>();
    range_vec.sort();
    let mut ranges = VecDeque::from(range_vec);
    let mut reduced_ranges: Vec<Range> = Vec::new();

    // reduce -- note: now for part 2 we have to merge everything to find the remaining non-blocked list
    // so iterate the approach for every next "first" element
    'outer: while !ranges.is_empty() {
        let mut newrange = ranges.pop_front().unwrap();
        while !ranges.is_empty() {
            let next = ranges.pop_front().unwrap();
            let new = newrange.merge(&next);
            if new.is_ok() {
                newrange = new.unwrap();
            } else {
                reduced_ranges.push(newrange);
                ranges.push_front(next);  // put back to queue
                continue 'outer;
            }
        }
        reduced_ranges.push(newrange);
    }

    // input data blocks the upper and lower bounds of the IP space
    // => do not require code for these edge cases
    assert_eq!(reduced_ranges[0].min, 0);
    assert_eq!(reduced_ranges.last().unwrap().max, u32::MAX);
    assert_eq!(4294967295, u32::MAX);

    // variant a) get all gaps
    // all gaps between the ranges
    let count = reduced_ranges.windows(2)
        .map(|w| {
            let (lower, higher) = (w[0], w[1]);
            let gap = higher.min - lower.max - 1;
            gap
        })
        .sum::<u32>();
    println!("numbers in range gaps: {}", count);

    // variant b) subtract sizes
    let range_size = reduced_ranges.iter().map(|r| r.size()).sum::<u32>();
    let count2 = u32::MAX - range_size + 1;
    println!("size of ranges: {} => diff: {}", range_size, count2);
    assert_eq!(count, count2);

    count.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 20)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("5-8
0-2
4-7")
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
    fn test_range_contains() {
        let r = Range::from_string("5-8");
        assert!(r.contains(5));
        assert!(r.contains(6));
        assert!(r.contains(7));
        assert!(r.contains(8));
        assert!(!r.contains(4));
        assert!(!r.contains(9));
    }

    #[test]
    fn test_range_size() {
        let r = Range{min: 5, max: 8};
        assert_eq!(r.size(), 4);
        let r = Range{min: 5, max: 5};
        assert_eq!(r.size(), 1);
    }

    #[test]
    fn test_range_merge() {
        let r = Range{min: 5, max: 8};
        let s = Range{min: 5, max: 5};
        assert_eq!(r.merge(&s).unwrap(), Range{min: 5, max: 8});

        let r = Range{min: 5, max: 8};
        let s = Range{min: 7, max: 12};
        assert_eq!(r.merge(&s).unwrap(), Range{min: 5, max: 12});

        let r = Range{min: 5, max: 8};
        let s = Range{min: 9, max: 12};
        assert_eq!(r.merge(&s).unwrap(), Range{min: 5, max: 12});

        let r = Range{min: 5, max: 6};
        let s = Range{min: 8, max: 9};
        assert!(r.merge(&s).is_err());
    }
}
