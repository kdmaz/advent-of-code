// https://adventofcode.com/2021/day/6

pub fn run_part1(path: &str) -> i32 {
    let content = util::get_file_content(path);
    let mut nums = content.split(",").map(|num| num.parse().unwrap()).collect::<Vec<i32>>();

    for _ in 0..80 {
        let mut zero_count = 0;
        nums = nums.iter().map(|num| {
            if num == &0 {
                zero_count += 1;
                6
            } else {
                num - 1
            }
        }).collect::<Vec<i32>>();

        if zero_count > 0 {
            for _ in 0..zero_count {
                nums.push(8);
            }
        }
    }

    nums.len() as i32
}

pub fn run_part2(path: &str) -> i32 {
    let _content = util::get_file_content(path);
    0
}

fn main() {}

mod tests {
    use crate::{run_part1, run_part2};

    #[test]
    fn part1_example() {
        assert_eq!(5934, run_part1("day06_example.txt"))
    }

    #[ignore]
    #[test]
    fn part2_example() {
        assert_eq!(-1, run_part2("day06_example.txt"))
    }

    #[test]
    fn part1() {
        assert_eq!(366057, run_part1("day06.txt"))
    }

    #[ignore]
    #[test]
    fn part2() {
        assert_eq!(-1, run_part2("day06.txt"))
    }
}
