
fn find_zero_digests(id: String, start: u32, zeroes: usize) -> (u32, char, char) {
    let mut i = start;
    let prefix = "0".repeat(zeroes);

    loop {
        let md5input = format!("{}{}", id, i);
        let digest = md5::compute(md5input.as_bytes());
        let hex_digest = format!("{:x}", digest);
        if hex_digest.starts_with(prefix.as_str()) {
            let char1 = hex_digest.chars().nth(zeroes).unwrap();
            let char2 = hex_digest.chars().nth(zeroes+1).unwrap();
            return (i, char1, char2)
        }
        i += 1
    }
}

fn solve_part1(input_text: String) -> String {
    let mut i: u32 = 0;
    let mut password: [char; 8] = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    for pw_i in 0..8 {
        let (found_i, pw_char, _) = find_zero_digests(input_text.clone(), i, 5);
        password[pw_i] = pw_char;
        i = found_i + 1;
    }
    password.into_iter().collect()
}

fn solve_part2(input_text: String) -> String {
    let mut i: u32 = 0;
    let mut password: [char; 8] = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    loop {
        let (found_i, pw_pos_char, pw_char) = find_zero_digests(input_text.clone(), i, 5);
        let pw_pos_result = pw_pos_char.to_string().parse::<usize>();
        match pw_pos_result {
            Ok(pw_pos) => {
                if pw_pos < password.len() && password[pw_pos] == ' ' {
                    password[pw_pos] = pw_char;
                    if !password.contains(&' ') { break }
                }
            }
            Err(_) => {}
        }
        i = found_i + 1;
    }
    password.into_iter().collect()
}


pub fn solve() {
    let input_text = String::from("ffykfhsq");

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let result = find_zero_digests(String::from("abc"), 3231900, 5);
        assert_eq!(result.0, 3231929);
        assert_eq!(result.1, '1');
        assert_eq!(result.2, '5');
    }
    #[test]
    fn test_part1b() {
        let result = find_zero_digests(String::from("abc"), 5017300, 5);
        assert_eq!(result.0, 5017308);
        assert_eq!(result.1, '8');
    }
    #[test]
    fn test_part1c() {
        let result = find_zero_digests(String::from("abc"), 5278500, 5);
        assert_eq!(result.0, 5278568);
        assert_eq!(result.1, 'f');
    }

    #[test]
    fn test_part2a() {
        let result = find_zero_digests(String::from("abc"), 5357500, 5);
        assert_eq!(result.0, 5357525);
        assert_eq!(result.1, '4');
        assert_eq!(result.2, 'e');
    }

    #[test]
    fn test_part2b() {
        let result = solve_part2(String::from("abc"));
        assert_eq!(result, "05ace8e3");
    }
}
