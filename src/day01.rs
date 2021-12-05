// https://adventofcode.com/2021/day/1

fn get_numbers(path: &str) -> Vec<i32> {
    util::get_file_content(path)
        .lines()
        .map(|l| {
            l.parse::<i32>()
                .expect(&format!("Could not convert &str ({}) to i32", l))
        })
        .collect()
}

trait Counter: Iterator {
    fn count_increments(self) -> i32
    where
        Self: Sized,
        Self::Item: Ord,
        Self::Item: Copy;
}

impl<I> Counter for I
where
    I: Iterator,
{
    fn count_increments(self) -> i32
    where
        Self: Sized,
        Self::Item: Ord,
        Self::Item: Copy,
    {
        self.fold((0, None), |(increased_count, previous), current| {
            if previous == None || current < previous.unwrap() || current == previous.unwrap() {
                (increased_count, Some(current))
            } else {
                (increased_count + 1, Some(current))
            }
        })
        .0
    }
}

pub fn run_part1(path: &str) -> i32 {
    get_numbers(path).iter().count_increments()
}

pub fn run_part2(path: &str) -> i32 {
    get_numbers(path)
        .windows(3)
        .map(|window| window.iter().sum::<i32>())
        .count_increments()
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{run_part1, run_part2};

    #[test]
    fn part1_example() {
        assert_eq!(7, run_part1("day01_example.txt"));
    }

    #[test]
    fn part2_example() {
        assert_eq!(5, run_part2("day01_example.txt"));
    }

    #[test]
    fn part1() {
        assert_eq!(1791, run_part1("day01.txt"));
    }

    #[test]
    fn part2() {
        assert_eq!(1822, run_part2("day01.txt"));
    }
}
