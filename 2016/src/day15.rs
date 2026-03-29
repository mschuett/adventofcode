use crate::helper;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Disc {
    id: u8,
    positions: u8,
    offset: u8,  // position at time 0
}
impl Disc {
    fn from_text(line: &str) -> Disc {
        let words = line
            .strip_prefix("Disc #").unwrap()
            .strip_suffix('.').unwrap()
            .split_whitespace().collect::<Vec<&str>>();
        assert_eq!(words.len(), 11);
        let id = words[0].parse::<u8>().unwrap();
        assert_eq!(words[1], "has");
        let positions = words[2].parse::<u8>().unwrap();
        assert_eq!(words[3], "positions;");
        assert_eq!(words[4], "at");
        assert_eq!(words[5], "time=0,");
        let offset = words[10].parse::<u8>().unwrap();
        Disc{id, positions, offset}
    }
    fn pos_for_drop_time(&self, time: u32) -> u8 {
        ((time + self.id as u32 + self.offset as u32) % self.positions as u32) as u8
    }
}

fn solve_naive(discs: Vec<Disc>) -> Result<u32, String> {
    // very naive simulation and checking, works for part 1
    for t in 0..u32::MAX {
        if discs.iter()
            .map(|d| d.pos_for_drop_time(t as u32))
            .all(|d| d == 0) {
            return Ok(t);
        }
    }
    Err("no solution found".to_string())
}

fn solve_improved(discs: Vec<Disc>) -> Result<u32, String> {
    /* we have a linear equation system with variable t;
       for every disc we want  0 = (t + id + offset) mod positions

       the positions are all prime, hence use their LCM as the common modulo
       then solve the linear equation system for t.
       for every disc we have a factor F=LCM/positions and we want
       0 = F*t + F*(id+offset)  mod LCM

       I fail to get an analytical solution due to the modulo, but with the LCM
       we have a relatively bounded range and we can check every t inside that.
     */
    let lcm = discs.iter()
        .map(|d| d.positions as u32)
        .product::<u32>();

    // pre-calc factor and const for every disc
    let factors = discs.iter()
        .map(|d| lcm/d.positions as u32)
        .collect::<Vec<u32>>();
    let consts = discs.iter()
        .map(|d| (lcm/d.positions as u32) * (d.id + d.offset) as u32)
        .collect::<Vec<u32>>();
    // full search
    for t in 0..lcm {
        if factors.iter()
            .enumerate()
            .all(|(i, &f)| (f as u64 * t as u64 + consts[i] as u64) % lcm as u64 == 0) {
            return Ok(t);
        }
    }
    Err("no solution found".to_string())
}

fn solve_part1(input_text: String) -> String {
    let discs = input_text.lines().map(Disc::from_text).collect::<Vec<Disc>>();
    println!("{:?}", discs);
    solve_improved(discs).unwrap().to_string()
}

fn solve_part2(input_text: String) -> String {
    let mut discs = input_text.lines().map(Disc::from_text).collect::<Vec<Disc>>();
    discs.push(Disc{id:1+discs.len() as u8, positions:11, offset:0});
    println!("{:?}", discs);
    solve_improved(discs).unwrap().to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 15)
            .expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from(
        "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.")
}
