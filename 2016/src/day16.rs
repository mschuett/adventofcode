use crate::helper;

fn gen_data_step(a: &str) -> String {
    let b = a
        .chars()
        .rev()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect::<String>();
    format!("{}0{}", a, b)
}

fn gen_data_fill(init: &str, size: usize) -> String {
    let mut data = init;
    let mut newdata: String;
    loop {
        newdata = gen_data_step(&data);
        if newdata.len() >= size {
            return newdata.get(..size).unwrap().to_string()
        }
        data = newdata.as_str().as_ref();
    }
}

fn checksum_step(data: &str) -> String {
    data
        .as_bytes()
        .chunks(2)
        .map(|c| if c[0] == c[1] { "1" } else { "0" })
        .collect::<String>()
}

fn checksum(data: &str) -> String {
    let mut workval = data;
    let mut newval: String;
    loop {
        newval = checksum_step(workval);
        if newval.len() % 2 == 1 {
            return newval
        } else {
            workval = newval.as_str();
        }
    }
}

fn solve_part1(input_text: String) -> String {
    let data = gen_data_fill(input_text.trim(), 272);
    assert_eq!(data.len(), 272);
    checksum(&data)
}

fn solve_part2(input_text: String) -> String {
    let data = gen_data_fill(input_text.trim(), 35651584);
    checksum(&data)
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 16)
            .expect("Could not fetch input");

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_data_step() {
        assert_eq!(gen_data_step("1"), "100");
        assert_eq!(gen_data_step("0"), "001");
        assert_eq!(gen_data_step("11111"), "11111000000");
        assert_eq!(gen_data_step("111100001010"), "1111000010100101011110000");
    }

    #[test]
    fn test_gen_data_fill() {
        assert_eq!(gen_data_fill("10000" ,20), "10000011110010000111");
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum("110010110100"), "100");
    }

    #[test]
    fn test_part1() {
        let data = gen_data_fill("10000", 20);
        assert_eq!(data, "10000011110010000111");
        let checksum = checksum(&data);
        assert_eq!(checksum, "01100");
    }
}
