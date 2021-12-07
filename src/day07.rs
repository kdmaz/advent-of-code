// https://adventofcode.com/2021/day/7

use std::collections::HashMap;

fn get_max_and_min(nums: &Vec<i32>) -> (isize, isize) {
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

    (*max.unwrap() as isize, *min.unwrap() as isize)
}

fn get_lowest_fuel_count(fuel_cost_by_position: &HashMap<isize, i32>) -> i32 {
    *fuel_cost_by_position.iter().fold(None, |cheapest_fuel_cost, (_, fuel_cost)| {
        if cheapest_fuel_cost == None || fuel_cost < cheapest_fuel_cost.unwrap() {
            Some(fuel_cost)
        } else {
            cheapest_fuel_cost
        }
    }).unwrap()
}

pub fn run_part1(path: &str) -> i32 {
    let content = util::get_file_content(path);
    let nums = content.split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let (max, min) = get_max_and_min(&nums);

    let mut fuel_cost_by_position = HashMap::new();

    for position in min..=max {
        let total_fuel_cost = nums.iter().fold(0, |total_fuel_cost, current_position| {
            total_fuel_cost + (current_position - position as i32).abs()
        });

        fuel_cost_by_position.insert(position, total_fuel_cost);
    }

    get_lowest_fuel_count(&fuel_cost_by_position)
}

pub fn run_part2(path: &str) -> i32 {
    let content = util::get_file_content(path);
    let nums = content.split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let (max, min) = get_max_and_min(&nums);

    let mut fuel_cost_by_position = HashMap::new();

    for position in min..=max {
        let total_fuel_cost = nums.iter().fold(0, |total_fuel_cost, current_position| {
            let difference = (current_position - position as i32).abs();
            total_fuel_cost + ((difference * (difference + 1)) / 2)
        });

        fuel_cost_by_position.insert(position, total_fuel_cost);
    }

    get_lowest_fuel_count(&fuel_cost_by_position)
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{run_part1, run_part2};

    #[test]
    fn part1_example() {
        assert_eq!(37, run_part1("day07_example.txt"))
    }

    #[test]
    fn part2_example() {
        assert_eq!(168, run_part2("day07_example.txt"))
    }

    #[test]
    fn part1() {
        assert_eq!(347011, run_part1("day07.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(98363777, run_part2("day07.txt"))
    }
}
