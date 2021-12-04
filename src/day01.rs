fn get_numbers() -> Vec<i32> {
    util::get_file_content("day01.txt")
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

pub fn run_part1() -> i32 {
    get_numbers().iter().count_increments()
}

pub fn run_part2() -> i32 {
    get_numbers()
        .windows(3)
        .map(|window| window.iter().sum::<i32>())
        .count_increments()
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{run_part1, run_part2};

    #[test]
    fn part1_correct() {
        assert_eq!(1791, run_part1());
    }

    #[test]
    fn part2_correct() {
        assert_eq!(1822, run_part2());
    }
}
