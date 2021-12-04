use std::collections::HashMap;

use crate::util;

pub fn run_part1() -> i32 {
    let mut ones_zeros_count_map = HashMap::new();

    let content = util::get_file_content("day03.txt");

    content.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            let contains_index = ones_zeros_count_map.contains_key(&i);

            if !contains_index {
                ones_zeros_count_map.insert(i, (0, 0));
            }

            let ones_zeros_count = ones_zeros_count_map.get(&i).unwrap();
            let ones_count = ones_zeros_count.0;
            let zeros_count = ones_zeros_count.1;

            match c {
                '0' => ones_zeros_count_map.insert(i, (ones_count, zeros_count + 1)),
                '1' => ones_zeros_count_map.insert(i, (ones_count + 1, zeros_count)),
                c @ _ => panic!("recieved a char other than a 1 or a 0: ({})", c),
            };
        })
    });

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for i in 0..12 {
        let (ones_count, zeros_count) = ones_zeros_count_map.get(&i).unwrap();

        if ones_count > zeros_count {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();
    gamma * epsilon
}

struct BitCount {
    zero_count: i32,
    one_count: i32,
}

fn get_bit_count_by_index(i: usize, lines: &Vec<&str>) -> BitCount {
    let mut bit_count = BitCount {
        zero_count: 0,
        one_count: 0,
    };

    lines.iter().for_each(|line| {
        let chars = line.chars().collect::<Vec<char>>();
        let bit = chars.get(i).expect(&format!(
            "index ({}) could not be found in line ({})",
            i, line
        ));
        match bit {
            '0' => bit_count.zero_count += 1,
            '1' => bit_count.one_count += 1,
            c @ _ => panic!("line contained a char other than 0 or 1 {}", c),
        }
    });

    bit_count
}

fn get_most_common_bit_by_index(i: usize, lines: &Vec<&str>) -> char {
    match get_bit_count_by_index(i, lines) {
        BitCount {
            zero_count,
            one_count,
        } if one_count == zero_count || one_count > zero_count => '1',
        _ => '0',
    }
}

fn get_least_common_bit_by_index(i: usize, lines: &Vec<&str>) -> char {
    match get_bit_count_by_index(i, lines) {
        BitCount {
            zero_count,
            one_count,
        } if zero_count == one_count || one_count > zero_count => '0',
        _ => '1',
    }
}

fn filter_lines_by_bits(
    mut lines: Vec<&str>,
    operation: fn(i: usize, lines: &Vec<&str>) -> char,
) -> i32 {
    let first_line = lines.first().expect("error getting first line of content");

    first_line.chars().enumerate().for_each(|(i, _)| {
        let bit_by_operation = operation(i, &lines);

        if lines.len() == 1 {
            return;
        }

        lines.retain(|line| {
            let bit = *line.chars().collect::<Vec<char>>().get(i).expect(&format!(
                "could not retrieve index ({}) from line ({})",
                i, line
            ));

            bit == bit_by_operation
        });
    });

    if lines.len() > 1 {
        panic!("more than one item remains in lines")
    } else if lines.is_empty() {
        panic!("no items remain in lines")
    }

    let bits = *lines
        .first()
        .expect("could not get the first item from lines");

    i32::from_str_radix(&bits, 2).expect(&format!("could not convert bits ({}) to integer", bits))
}

pub fn run_part2() -> i32 {
    let content = util::get_file_content("day03.txt");
    let lines = content.lines().collect::<Vec<&str>>();
    let lines_copy = lines.to_vec();

    let oxygen_generator = filter_lines_by_bits(lines, get_most_common_bit_by_index);
    let co2_scrubber = filter_lines_by_bits(lines_copy, get_least_common_bit_by_index);

    oxygen_generator * co2_scrubber
}

#[cfg(test)]
mod tests {
    use crate::day03::{run_part1, run_part2};

    #[test]
    fn part1_correct() {
        assert_eq!(775304, run_part1());
    }

    #[test]
    fn part2_correct() {
        assert_eq!(1370737, run_part2());
    }
}
