// https://adventofcode.com/2021/day/5

use std::{cmp, collections::HashMap};

#[derive(Debug)]
struct VentLines {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn get_vent_lines(path: &str) -> Vec<VentLines> {
    util::get_file_content(path)
        .lines()
        .map(|line| {
            let parse_int = |line: Option<&str>| line.unwrap().trim().parse::<i32>().unwrap();

            let mut line = line.splitn(2, ",");
            let x1 = parse_int(line.next());
            let mut line = line.next().unwrap().splitn(2, "->");
            let y1 = parse_int(line.next());
            let mut line = line.next().unwrap().splitn(2, ",");
            let x2 = parse_int(line.next());
            let y2 = parse_int(line.next());

            VentLines { x1, y1, x2, y2 }
        })
        .collect::<Vec<VentLines>>()
}

type PointsMap = HashMap<(i32, i32), i32>;

fn get_points_map(vent_lines: &Vec<VentLines>) -> PointsMap {
    let mut points_map = HashMap::new();

    for VentLines { x1, y1, x2, y2 } in vent_lines {
        let same_points = x1 == x2 && y1 == y2;
        let points_mismatch = x1 != x2 && y1 != y2;

        if points_mismatch || same_points {
            continue;
        }

        let bigger_x = *cmp::max(x1, x2) as isize;
        let smaller_x = *cmp::min(x1, x2) as isize;
        let bigger_y = *cmp::max(y1, y2) as isize;
        let smaller_y = *cmp::min(y1, y2) as isize;

        let update_points_map = |points_map: &mut PointsMap, position| {
            let count = points_map.entry(position).or_insert(0);
            *count += 1;
        };

        if x1 == x2 {
            let x = *x1;

            for y in smaller_y..=bigger_y {
                update_points_map(&mut points_map, (x, y as i32));
            }
        } else if y1 == y2 {
            let y = *y1;

            for x in smaller_x..=bigger_x {
                update_points_map(&mut points_map, (x as i32, y));
            }
        }
    }

    points_map
}

fn get_dangerous_point_count(points_map: &PointsMap) -> i32 {
    points_map
        .iter()
        .filter(|(_, count)| count >= &&2)
        .map(|_| 1)
        .sum()
}

pub fn run_part1(path: &str) -> i32 {
    let vent_lines = get_vent_lines(path);
    let points_map = get_points_map(&vent_lines);
    get_dangerous_point_count(&points_map)
}

fn get_points_map_with_diagnal(vent_lines: &Vec<VentLines>) -> PointsMap {
    let mut points_map = HashMap::new();

    for VentLines { x1, y1, x2, y2 } in vent_lines {
        let same_points = x1 == x2 && y1 == y2;
        let points_mismatch = x1 != x2 && y1 != y2;
        let not_diagnal = (x1 - x2).abs() != (y1 - y2).abs();

        if (points_mismatch && not_diagnal) || same_points {
            continue;
        }

        let bigger_x = *cmp::max(x1, x2) as isize;
        let smaller_x = *cmp::min(x1, x2) as isize;
        let bigger_y = *cmp::max(y1, y2) as isize;
        let smaller_y = *cmp::min(y1, y2) as isize;

        let update_points_map = |points_map: &mut PointsMap, position| {
            let count = points_map.entry(position).or_insert(0);
            *count += 1;
        };

        if x1 == x2 {
            let x = *x1;

            for y in smaller_y..=bigger_y {
                update_points_map(&mut points_map, (x, y as i32));
            }
        } else if y1 == y2 {
            let y = *y1;

            for x in smaller_x..=bigger_x {
                update_points_map(&mut points_map, (x as i32, y));
            }
        } else if (x1 < x2 && y1 > y2) || (x2 < x1 && y2 > y1) {
            // / Secondary Diagnal /
            for i in 0..=bigger_x - smaller_x {
                let x = (smaller_x + i) as i32;
                let y = (bigger_y - i) as i32;

                update_points_map(&mut points_map, (x, y));
            }
        } else {
            // \ Primary Diagnal \
            for i in 0..=bigger_x - smaller_x {
                let x = (smaller_x + i) as i32;
                let y = (smaller_y + i) as i32;

                update_points_map(&mut points_map, (x, y));
            }
        }
    }

    points_map
}

pub fn run_part2(path: &str) -> i32 {
    let vent_lines = get_vent_lines(path);
    let points_map = get_points_map_with_diagnal(&vent_lines);
    get_dangerous_point_count(&points_map)
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{get_dangerous_point_count, run_part1, run_part2};

    #[test]
    fn part1_example() {
        assert_eq!(5, run_part1("day05_example.txt"))
    }

    #[test]
    fn part2_example() {
        assert_eq!(12, run_part2("day05_example.txt"))
    }

    #[test]
    fn part1() {
        assert_eq!(5145, run_part1("day05.txt"))
    }

    #[test]
    fn part2() {
        assert_eq!(16518, run_part2("day05.txt"))
    }

    #[test]
    fn dangerous_points() {
        let mut points_map = HashMap::new();

        // row 0
        points_map.insert((7, 0), 1);

        // row 1
        points_map.insert((2, 1), 1);
        points_map.insert((7, 1), 1);

        // row 2
        points_map.insert((2, 2), 1);
        points_map.insert((7, 2), 1);

        // row 3
        points_map.insert((7, 3), 1);

        // row 4
        points_map.insert((1, 4), 1);
        points_map.insert((2, 4), 1);
        points_map.insert((3, 4), 2);
        points_map.insert((4, 4), 1);
        points_map.insert((5, 4), 1);
        points_map.insert((6, 4), 1);
        points_map.insert((7, 4), 2);
        points_map.insert((8, 4), 1);
        points_map.insert((9, 4), 1);

        // row 9
        points_map.insert((0, 9), 2);
        points_map.insert((1, 9), 2);
        points_map.insert((2, 9), 2);
        points_map.insert((3, 9), 1);
        points_map.insert((4, 9), 1);
        points_map.insert((5, 9), 1);

        assert_eq!(get_dangerous_point_count(&points_map), 5);
    }
}
