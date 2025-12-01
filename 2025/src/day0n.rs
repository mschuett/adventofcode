use crate::helper;

fn solve_part1(input_text: String) -> String {
    let _input_example = "";
    !todo("write me");
}

fn solve_part2(input_text: String) -> String {
    let _input_example = "";
    !todo("write me");
}

pub fn solve() {
    let example = true;
    let input_text = if example {
        String::from(
            "")
    } else {
        helper::fetch_cache_input_text(2024, 1)
            .expect("Could not fetch input")
    };

    print!("Part 1: {}\nPart 2: {}\n",
           solve_part1(input_text.clone()),
           solve_part2(input_text));
}
