use std::fs;
const FILENAME: &str = "day01.txt";

fn get_lines() -> String {
    return fs::read_to_string(FILENAME)
        .expect(&format!("Could not read from file ({})", FILENAME));
}

pub fn run_part1() {
    let increased_count = get_lines()
        .split_whitespace()
        .map(|l| {
            l.parse::<i32>()
                .expect(&format!("Could not convert &str ({}) to i32", l))
        })
        .fold((0, None), |(increased_count, previous), current| {
            if previous == None || current < previous.unwrap() {
                (increased_count, Some(current))
            } else {
                (increased_count + 1, Some(current))
            }
        })
        .0;

    println!("day 1, part 1: {}", increased_count);
}
