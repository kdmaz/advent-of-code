// https://adventofcode.com/2021/day/

pub fn run_part1(path: &str) -> i32 {
    let content = util::get_file_content(path);
    0
}

pub fn run_part2(path: &str) -> i32 {
    let content = util::get_file_content(path);
    0
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{run_part1, run_part2};

    #[ignore]
    #[test]
    fn part1_example() {
        assert_eq!(-1, run_part1("day00_example.txt"))
    }

    #[ignore]
    #[test]
    fn part2_example() {
        assert_eq!(-1, run_part2("day00_example.txt"))
    }

    #[ignore]
    #[test]
    fn part1() {
        assert_eq!(-1, run_part1("day00.txt"))
    }

    #[ignore]
    #[test]
    fn part2() {
        assert_eq!(-1, run_part2("day00.txt"))
    }
}
