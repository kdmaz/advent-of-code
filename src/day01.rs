use std::fs;
const FILENAME: &str = "day01.txt";

fn get_file_content() -> String {
    return fs::read_to_string(FILENAME)
        .expect(&format!("Could not read from file ({})", FILENAME));
}

fn to_numbers<'a>(content: &'a String) -> impl Iterator<Item = i32> + 'a {
    content.lines().map(|l| {
        l.parse::<i32>()
            .expect(&format!("Could not convert &str ({}) to i32", l))
    })
}

fn count_increments(
    (increased_count, previous): (i32, Option<i32>),
    current: i32,
) -> (i32, Option<i32>) {
    if previous == None || current < previous.unwrap() || current == previous.unwrap() {
        (increased_count, Some(current))
    } else {
        (increased_count + 1, Some(current))
    }
}

pub fn run_part1() -> i32 {
    let content = get_file_content();
    return to_numbers(&content).fold((0, None), count_increments).0;
}

pub fn run_part2() -> i32 {
    let content = get_file_content();
    let numbers: Vec<i32> = to_numbers(&content).collect();
    return numbers
        .windows(3)
        .map(|window| window.iter().sum::<i32>())
        .fold((0, None), count_increments)
        .0;
}

#[cfg(test)]
mod tests {
    use crate::day01::{run_part1, run_part2};

    #[test]
    fn part1_correct() {
        assert_eq!(1791, run_part1());
    }

    #[test]
    fn part2_correct() {
        assert_eq!(1822, run_part2());
    }
}
