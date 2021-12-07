use std::collections::HashMap;

// https://adventofcode.com/2021/day/7

pub fn run_part1(path: &str) -> i32 {
    let content = util::get_file_content(path);
    let nums = content.split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let (max, min) = nums.iter().fold((None, None), |(max, min), current_position| {
        if max == None || min == None {
            return (Some(current_position), Some(current_position));
        }

        let max_and_min = (max, min);

        if current_position > max.unwrap() {
            (Some(current_position), min)
        } else if current_position < min.unwrap() {
            (max, Some(current_position))
        } else {
            max_and_min
        }
    });

    let (max, min) = (*max.unwrap() as isize, *min.unwrap() as isize);

    let mut fuel_cost_by_position = HashMap::new();

    for position in min..=max {
        let fuel_cost = nums.iter().fold(0, |fuel_cost, current_position| {
            fuel_cost + (current_position - position as i32).abs()
        });

        fuel_cost_by_position.insert(position, fuel_cost);
    }

    *fuel_cost_by_position.iter().fold(None, |cheapest_fuel_cost, (_, fuel_cost)| {
        if cheapest_fuel_cost == None || fuel_cost < cheapest_fuel_cost.unwrap() {
            Some(fuel_cost)
        } else {
            cheapest_fuel_cost
        }
    }).unwrap()
}

pub fn run_part2(path: &str) -> i32 {
    0
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{run_part1, run_part2};

    #[test]
    fn part1_example() {
        assert_eq!(37, run_part1("day07_example.txt"))
    }

    #[ignore]
    #[test]
    fn part2_example() {
        assert_eq!(-1, run_part2("day07_example.txt"))
    }

    #[test]
    fn part1() {
        assert_eq!(347011, run_part1("day07.txt"))
    }

    #[ignore]
    #[test]
    fn part2() {
        assert_eq!(-1, run_part2("day07.txt"))
    }
}
