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
        line.next()
            .unwrap()
            .trim()
            .split_whitespace()
            .for_each(|segment| {
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

fn get_char_by_position(unique_signal_pattern: Vec<&str>) -> HashMap<&str, char> {
    let mut char_by_position = HashMap::new();

    let mut one_chars_set = HashSet::new();
    let mut seven_chars_set = HashSet::new();
    let mut eight_chars_set = HashSet::new();
    for segment in &unique_signal_pattern {
        if segment.len() == 2 {
            segment.chars().for_each(|c| {
                one_chars_set.insert(c);
            });
        }

        if segment.len() == 3 {
            segment.chars().for_each(|c| {
                seven_chars_set.insert(c);
            });
        }

        if segment.len() == 7 {
            segment.chars().for_each(|c| {
                eight_chars_set.insert(c);
            });
        }
    }

    for segment in &unique_signal_pattern {
        if segment.len() == 2 {
            let top = **seven_chars_set
                .difference(&one_chars_set)
                .collect::<HashSet<_>>()
                .iter()
                .next()
                .unwrap();

            char_by_position.insert("top", top);
        }
    }

    for segment in &unique_signal_pattern {
        if segment.len() == 6 {
            let mut temp_set = HashSet::new();
            segment.chars().for_each(|c| {
                temp_set.insert(c);
            });

            if !one_chars_set.is_subset(&temp_set) {
                let top_right = **one_chars_set
                    .difference(&temp_set)
                    .collect::<HashSet<_>>()
                    .iter()
                    .next()
                    .unwrap();

                char_by_position.insert("top_right", top_right);

                let mut temp_set = HashSet::new();
                temp_set.insert(top_right);

                let bottom_right = **one_chars_set
                    .difference(&temp_set)
                    .collect::<HashSet<_>>()
                    .iter()
                    .next()
                    .unwrap();

                char_by_position.insert("bottom_right", bottom_right);
            }
        }
    }

    let mut two_chars_set = HashSet::new();
    let mut three_chars_set = HashSet::new();
    let mut five_chars_set = HashSet::new();
    for segment in &unique_signal_pattern {
        if segment.len() == 5 {
            let mut temp_set = HashSet::new();
            segment.chars().for_each(|c| {
                temp_set.insert(c);
            });

            if one_chars_set.is_subset(&temp_set) {
                three_chars_set = temp_set.clone();
            } else {
                if temp_set.contains(char_by_position.get("top_right").unwrap()) {
                    two_chars_set = temp_set.clone();
                } else {
                    five_chars_set = temp_set.clone();
                }
            }
        }
    }

    for segment in &unique_signal_pattern {
        if segment.len() == 5 {
            for c in segment.chars() {
                if two_chars_set.contains(&c) && !three_chars_set.contains(&c) {
                    char_by_position.insert("bottom_left", c);
                } else if five_chars_set.contains(&c) && !three_chars_set.contains(&c) {
                    char_by_position.insert("top_left", c);
                }
            }
        }
    }

    for segment in &unique_signal_pattern {
        if segment.len() == 4 {
            for c in segment.chars() {
                if &c != char_by_position.get("top_left").unwrap()
                    && &c != char_by_position.get("top_right").unwrap()
                    && &c != char_by_position.get("bottom_right").unwrap()
                {
                    char_by_position.insert("middle", c);
                }
            }
        }
    }

    for segment in &unique_signal_pattern {
        if segment.len() == 7 {
            let mut temp_set = HashSet::new();
            for (_, c) in &char_by_position {
                temp_set.insert(*c);
            }

            let bottom = **eight_chars_set
                .difference(&temp_set)
                .collect::<HashSet<_>>()
                .iter()
                .next()
                .unwrap();

            char_by_position.insert("bottom", bottom);
        }
    }

    char_by_position
}

pub fn run_part2(path: &str) -> i32 {
    let content = get_formatted_content(path);

    content.lines().fold(0, |total_value_count, line| {
        let mut line = line.split("|");

        let unique_signal_pattern = line
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();

        let char_by_position = get_char_by_position(unique_signal_pattern);

        let segment_not_contain_position = |segment: &str, position: &str| {
            !segment.contains(|c| char_by_position.get(position).unwrap() == &c)
        };

        let value = line
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|segment| match segment.len() {
                2 => "1",
                4 => "4",
                3 => "7",
                7 => "8",
                6 => {
                    if segment_not_contain_position(segment, "middle") {
                        "0"
                    } else if segment_not_contain_position(segment, "top_right") {
                        "6"
                    } else {
                        "9"
                    }
                }
                5 => {
                    if segment_not_contain_position(segment, "top_right") {
                        "5"
                    } else if segment_not_contain_position(segment, "bottom_right") {
                        "2"
                    } else {
                        "3"
                    }
                }
                l @ _ => panic!("length ({}) did not match anything", l),
            })
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

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

    #[test]
    fn part2() {
        assert_eq!(915941, run_part2("day08.txt"))
    }
}
