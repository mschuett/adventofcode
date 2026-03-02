use crate::helper;

struct Triangle {
    a: u32,
    b: u32,
    c: u32,
}
impl Triangle {
    fn from_vec(vec: Vec<u32>) -> Self {
        Triangle{a: vec[0], b: vec[1], c: vec[2]}
    }
    fn from_line(line: &str) -> Self {
        let vec = line.split_whitespace()
            .into_iter()
            .map(|n| n.parse::<u32>().unwrap() )
            .collect::<Vec<u32>>();
        Triangle::from_vec(vec)
    }
    pub fn is_possible(&self) -> bool {
        self.a + self.b > self.c && self.a + self.c > self.b && self.b + self.c > self.a
    }
}

fn solve_part1(input_text: String) -> String {
    let mut possible: u32 = 0;
    for line in input_text.lines() {
        let t = Triangle::from_line(line);
        if t.is_possible() { possible += 1 }
    }
    possible.to_string()
}

fn solve_part2(input_text: String) -> String {
    let mut vec1: Vec<u32> = Vec::new();
    let mut vec2: Vec<u32> = Vec::new();
    let mut vec3: Vec<u32> = Vec::new();
    for line in input_text.lines() {
        let line_vec = line.split_whitespace()
            .into_iter()
            .map(|n| n.parse::<u32>().unwrap() )
            .collect::<Vec<u32>>();
        vec1.push(line_vec[0]);
        vec2.push(line_vec[1]);
        vec3.push(line_vec[2]);
    }
    vec1.extend(vec2);
    vec1.extend(vec3);
    vec1.chunks(3)
        .map(|vec| { Triangle{a: vec[0], b: vec[1], c: vec[2] } })
        .filter(|t| t.is_possible())
        .count().to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 3)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("5 10 25")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "0");
    }
    // no real example for part 2
}
