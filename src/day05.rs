use std::collections::HashMap;

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
    HashMap::new()
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
        assert_eq!(5, run_part1())
    }

    // #[test]
    // fn part2_correct() {}

    #[test]
    fn dangerous_points() {
        let mut points_map = HashMap::new();

        // row 0
        points_map.insert((0, 0), 0);
        points_map.insert((1, 0), 0);
        points_map.insert((2, 0), 0);
        points_map.insert((3, 0), 0);
        points_map.insert((4, 0), 0);
        points_map.insert((5, 0), 0);
        points_map.insert((6, 0), 0);
        points_map.insert((7, 0), 1);
        points_map.insert((8, 0), 0);
        points_map.insert((9, 0), 0);

        // row 1
        points_map.insert((0, 1), 0);
        points_map.insert((1, 1), 0);
        points_map.insert((2, 1), 1);
        points_map.insert((3, 1), 0);
        points_map.insert((4, 1), 0);
        points_map.insert((5, 1), 0);
        points_map.insert((6, 1), 0);
        points_map.insert((7, 1), 1);
        points_map.insert((8, 1), 0);
        points_map.insert((9, 1), 0);

        // row 2
        points_map.insert((0, 2), 0);
        points_map.insert((1, 2), 0);
        points_map.insert((2, 2), 1);
        points_map.insert((3, 2), 0);
        points_map.insert((4, 2), 0);
        points_map.insert((5, 2), 0);
        points_map.insert((6, 2), 0);
        points_map.insert((7, 2), 1);
        points_map.insert((8, 2), 0);
        points_map.insert((9, 2), 0);

        // row 3
        points_map.insert((0, 3), 0);
        points_map.insert((1, 3), 0);
        points_map.insert((2, 3), 0);
        points_map.insert((3, 3), 0);
        points_map.insert((4, 3), 0);
        points_map.insert((5, 3), 0);
        points_map.insert((6, 3), 0);
        points_map.insert((7, 3), 1);
        points_map.insert((8, 3), 0);
        points_map.insert((9, 3), 0);

        // row 4
        points_map.insert((0, 4), 0);
        points_map.insert((1, 4), 1);
        points_map.insert((2, 4), 1);
        points_map.insert((3, 4), 2);
        points_map.insert((4, 4), 1);
        points_map.insert((5, 4), 1);
        points_map.insert((6, 4), 1);
        points_map.insert((7, 4), 2);
        points_map.insert((8, 4), 1);
        points_map.insert((9, 4), 1);

        // row 5
        points_map.insert((0, 5), 0);
        points_map.insert((1, 5), 0);
        points_map.insert((2, 5), 0);
        points_map.insert((3, 5), 0);
        points_map.insert((4, 5), 0);
        points_map.insert((5, 5), 0);
        points_map.insert((6, 5), 0);
        points_map.insert((7, 5), 0);
        points_map.insert((8, 5), 0);
        points_map.insert((9, 5), 0);

        // row 6
        points_map.insert((0, 6), 0);
        points_map.insert((1, 6), 0);
        points_map.insert((2, 6), 0);
        points_map.insert((3, 6), 0);
        points_map.insert((4, 6), 0);
        points_map.insert((5, 6), 0);
        points_map.insert((6, 6), 0);
        points_map.insert((7, 6), 0);
        points_map.insert((8, 6), 0);
        points_map.insert((9, 6), 0);

        // row 7
        points_map.insert((0, 7), 0);
        points_map.insert((1, 7), 0);
        points_map.insert((2, 7), 0);
        points_map.insert((3, 7), 0);
        points_map.insert((4, 7), 0);
        points_map.insert((5, 7), 0);
        points_map.insert((6, 7), 0);
        points_map.insert((7, 7), 0);
        points_map.insert((8, 7), 0);
        points_map.insert((9, 7), 0);

        // row 8
        points_map.insert((0, 8), 0);
        points_map.insert((1, 8), 0);
        points_map.insert((2, 8), 0);
        points_map.insert((3, 8), 0);
        points_map.insert((4, 8), 0);
        points_map.insert((5, 8), 0);
        points_map.insert((6, 8), 0);
        points_map.insert((7, 8), 0);
        points_map.insert((8, 8), 0);
        points_map.insert((9, 8), 0);

        // row 9
        points_map.insert((0, 9), 2);
        points_map.insert((1, 9), 2);
        points_map.insert((2, 9), 2);
        points_map.insert((3, 9), 1);
        points_map.insert((4, 9), 1);
        points_map.insert((5, 9), 1);
        points_map.insert((6, 9), 0);
        points_map.insert((7, 9), 0);
        points_map.insert((8, 9), 0);
        points_map.insert((9, 9), 0);

        assert_eq!(get_dangerous_point_count(&points_map), 5);
    }
}
