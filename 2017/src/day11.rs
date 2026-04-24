use crate::helper;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum HexDirs {
    NW, N, NE, SE, S, SW
}
impl TryFrom<&str> for HexDirs {
    type Error = String;
    fn try_from(s: &str) -> Result<HexDirs, Self::Error> {
        match s {
            "nw" => Ok(HexDirs::NW),
            "n" => Ok(HexDirs::N),
            "ne" => Ok(HexDirs::NE),
            "sw" => Ok(HexDirs::SW),
            "s" => Ok(HexDirs::S),
            "se" => Ok(HexDirs::SE),
            _ => Err(format!("unknown direction {}", s)),
        }
    }
}
// representing the hexgrid with Cube coordinates
// https://www.redblobgames.com/grids/hexagons/#coordinates-cube
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct HexCoord {
    q: i32,
    r: i32,
    s: i32,
}
impl HexCoord {
    fn step(&self, dir: HexDirs) -> HexCoord {
        match dir {
            HexDirs::NW => HexCoord { q: self.q - 1, r: self.r,     s: self.s + 1 },
            HexDirs::N  => HexCoord { q: self.q,     r: self.r - 1, s: self.s + 1 },
            HexDirs::NE => HexCoord { q: self.q + 1, r: self.r - 1, s: self.s     },
            HexDirs::SW => HexCoord { q: self.q - 1, r: self.r + 1, s: self.s     },
            HexDirs::S  => HexCoord { q: self.q,     r: self.r + 1, s: self.s - 1},
            HexDirs::SE => HexCoord { q: self.q + 1, r: self.r,     s: self.s - 1 },
        }
    }
    fn distance(&self, other: &HexCoord) -> u32 {
        ((self.q - other.q).abs() as u32
            + (self.r - other.r).abs() as u32
            + (self.s - other.s).abs() as u32
        ) / 2
    }
}

fn solve_part1(input_text: String) -> String {
    let steps: Vec<HexDirs> = input_text.trim()
        .split(",")
        .map(HexDirs::try_from)
        .collect::<Result<_, _>>()
        .unwrap();
    let init = HexCoord { q: 0, r: 0, s: 0 };
    let mut cursor = init.clone();
    for dir in steps {
        cursor = cursor.step(dir);
    }
    cursor.distance(&init).to_string()
}

fn solve_part2(input_text: String) -> String {
    let steps: Vec<HexDirs> = input_text.trim()
        .split(",")
        .map(HexDirs::try_from)
        .collect::<Result<_, _>>()
        .unwrap();
    let init = HexCoord { q: 0, r: 0, s: 0 };
    let mut cursor = init.clone();
    let mut max_dist: u32 = 0;
    for dir in steps {
        cursor = cursor.step(dir);
        let dist = cursor.distance(&init);
        if dist > max_dist {
            max_dist = dist
        }
    }
    max_dist.to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2017, 11)
        .expect("Could not fetch input");

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("ne,ne,ne".to_string()), "3");
        assert_eq!(solve_part1("ne,ne,sw,sw".to_string()), "0");
        assert_eq!(solve_part1("ne,ne,s,s".to_string()), "2");
        assert_eq!(solve_part1("se,sw,se,sw,sw".to_string()), "3");
    }
}
