use std::collections::HashMap;

use crate::util;

pub fn run_part1() {
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

    let gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("{}", gamma * epsilon);
}
