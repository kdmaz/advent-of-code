use std::collections::HashMap;

// https://adventofcode.com/2021/day/6

pub fn run_part1(path: &str, days: i32) -> i64 {
    let content = util::get_file_content(path);
    let mut nums_map: HashMap<i32, i64> = HashMap::new();

    content.split(",").for_each(|num| {
        let count = nums_map.entry(num.parse().unwrap()).or_insert(0);
        *count += 1;
    });

    let mut temp_map = HashMap::new();
    for _ in 0..days {
        for (day, count) in nums_map {
            if day == 0 {
                let c1 = temp_map.entry(6).or_insert(0);
                *c1 += count;

                let c2 = temp_map.entry(8).or_insert(0);
                *c2 += count;
            } else {
                let c = temp_map.entry(day - 1).or_insert(0);
                *c += count;
            }
        }
        nums_map = temp_map.clone();
        temp_map.clear();
    }

    nums_map.iter().fold(0, |count, (_, v)| count + v)
}

pub fn run_part2(path: &str, days: i32) -> i64 {
    run_part1(path, days)
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{run_part1, run_part2};

    #[test]
    fn part1_example() {
        assert_eq!(5934, run_part1("day06_example.txt", 80))
    }

    #[test]
    fn part2_example() {
        assert_eq!(26984457539, run_part2("day06_example.txt", 256))
    }

    #[test]
    fn part1() {
        assert_eq!(366057, run_part1("day06.txt", 80))
    }

    #[test]
    fn part2() {
        assert_eq!(1653559299811, run_part2("day06.txt", 256))
    }
}
