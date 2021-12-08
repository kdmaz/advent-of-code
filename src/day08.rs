// https://adventofcode.com/2021/day/8

use std::collections::{HashMap, HashSet};


fn get_formatted_content(path: &str) -> String {
	let content = util::get_file_content(path);

	if cfg!(windows) {
		content.replace("|\r\n", "| ")
	} else {
		content.replace("|\n", "| ")
	}	
}

pub fn run_part1(path: &str) -> i32 {
	let content = get_formatted_content(path);
	let mut segment_len_map = HashMap::new();
	
	content.lines().for_each(|line| {
		let mut line = line.split("|");
		line.next();
		line.next().unwrap().trim().split_whitespace().for_each(|segment| {
			let count = segment_len_map.entry(segment.len()).or_insert(0);
			*count += 1;
		});
	});

	let one_count = segment_len_map.get(&2).unwrap();
	let four_count = segment_len_map.get(&4).unwrap();
	let seven_count = segment_len_map.get(&3).unwrap();
	let eight_count = segment_len_map.get(&7).unwrap();

	one_count + four_count + seven_count + eight_count
}

pub fn run_part2(path: &str) -> i32 {
	let content = get_formatted_content(path);

	content.lines().fold(0, |total_value_count, line| {
		let mut line = line.split("|");

		// let char_by_position = HashMap::new();

		line.next().unwrap().trim().split_whitespace();

		let value = line.next().unwrap().trim().split_whitespace().map(|segment| {
			match segment.len() {
				2 => return "1",
				4 => return "4",
				3 => return "3",
				7 => return "8",
				_ => {}
			}

			let mut sorted_segment = segment.chars().collect::<Vec<char>>();
			sorted_segment.sort();
			let sorted_segment = sorted_segment.iter().collect::<String>();

			match sorted_segment.as_str() {
				"ab" => "1",
				"acdfg" => "2",
				"abcdf" => "3",
				"abef" => "4",
				"bcdef" => "5",
				"bcdefg" => "6",
				"abd" => "7",
				"abcdefg" => "8",
				"abcdef" => "9",
				"abcdeg" => "0",
				c @ _ => panic!("could not find digit that matches ({})", c)
			}
		}).collect::<String>().parse::<i32>().unwrap();
		total_value_count + value
	})
}

fn main() {}

#[cfg(test)]
mod tests {
	use crate::{run_part1, run_part2};

	#[test]
	fn part1_example() {
			assert_eq!(26, run_part1("day08_example.txt"))
	}

	#[test]
	fn part2_example() {
			assert_eq!(61229, run_part2("day08_example.txt"))
	}

	#[test]
	fn part1() {
			assert_eq!(310, run_part1("day08.txt"))
	}

	#[ignore]
	#[test]
	fn part2() {
			assert_eq!(-1, run_part2("day08.txt"))
	}
}
