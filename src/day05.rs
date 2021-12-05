use std::{cmp, collections::HashMap};

#[derive(Debug)]
struct VentLines {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn get_vent_lines() -> Vec<VentLines> {
    util::get_file_content("day05.txt")
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
        if x1 != x2 && y1 != y2 || (x1 == x2 && y1 == y2) {
            continue;
        }

        if x1 == x2 {
            let bigger = *cmp::max(y1, y2) as isize;
            let smaller = *cmp::min(y1, y2) as isize;
            let x = *x1;

            for y in smaller..=bigger {
                let count = points_map.entry((x, y as i32)).or_insert(0);
                *count += 1;
            }
        } else if y1 == y2 {
            let bigger = *cmp::max(x1, x2) as isize;
            let smaller = *cmp::min(x1, x2) as isize;
            let y = *y1;

            for x in smaller..=bigger {
                let count = points_map.entry((x as i32, y)).or_insert(0);
                *count += 1;
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

pub fn run_part1() -> i32 {
    let vent_lines = get_vent_lines();
    let points_map = get_points_map(&vent_lines);
    get_dangerous_point_count(&points_map)
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{get_dangerous_point_count, run_part1};

    #[test]
    fn part1_correct() {
        assert_eq!(5145, run_part1())
    }

    // #[test]
    // fn part2_correct() {}

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
