mod day01;
mod helper;

fn main() {
    let input_text = helper::fetch_cache_input_text(2024, 1).expect("Could not fetch input");
    let result = day01::solve(input_text);
    println!("{}", result);
}
