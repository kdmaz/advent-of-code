use std::collections::HashMap;

// https://adventofcode.com/2021/day/8

pub fn run_part1(path: &str) -> i32 {
	let content = util::get_file_content(path);

	// 10 | 4
	let x;

	if cfg!(windows) {
		x = content.replace("|\r\n", "| ");
	} else {
		x = content.replace("|\n", "| ");
	}

	let mut segment_len_map = HashMap::new();

	x.lines().for_each(|line| {
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
	0
}

fn main() {}

#[cfg(test)]
mod tests {
	use crate::{run_part1, run_part2};

	#[test]
	fn part1_example() {
			assert_eq!(26, run_part1("day08_example.txt"))
	}

	#[ignore]
	#[test]
	fn part2_example() {
			assert_eq!(-1, run_part2("day08_example.txt"))
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
