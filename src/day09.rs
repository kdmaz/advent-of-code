// https://adventofcode.com/2021/day/9

use std::{hash::Hash, collections::HashSet};

pub fn run_part1(path: &str) -> i32 {
    let content = util::get_file_content(path);
    let nums_by_line = content
        .lines()
        .map(|l| {
            l.chars()
                .map(|num| num.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut lowest_points = Vec::new();

    let mut row = 0;
    for nums in &nums_by_line {
        for (i, num) in nums.iter().enumerate() {
            let left = if (i as i32 - 1) < 0 {
                None
            } else {
                nums.get(i - 1)
            };

            let right = if (i as i32 + 1) < 0 {
                None
            } else {
                nums.get(i + 1)
            };

            let up = if (row as i32 - 1) < 0 {
                None
            } else {
                match nums_by_line.get(row - 1) {
                    Some(row) => row.get(i),
                    None => None,
                }
            };

            let down = if (row as i32 + 1) < 0 {
                None
            } else {
                match nums_by_line.get(row + 1) {
                    Some(row) => row.get(i),
                    None => None,
                }
            };

            let is_less = |x: Option<&i32>| x.is_none() || x.unwrap() > num;

            if is_less(left) && is_less(right) && is_less(up) && is_less(down) {
                lowest_points.push(*num);
            }
        }
        row += 1;
    }

    lowest_points
        .iter()
        .fold(0, |count, point| count + point + 1)
}

#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
struct Point {
    val: i32,
    x: i32,
    y: i32,
}

fn recursively_add_to_basin(basin: &mut HashSet<Point>, point: Point, row: &Vec<i32>, table: &Vec<Vec<i32>>) {
    if basin.contains(&point) {
        return;
    }

    basin.insert(point);

    let Point { x, y, ..} = point;

    // left
    if (x - 1) >= 0 {
        let left = row.get(x as usize - 1);
        if left.is_some() {
            recursively_add_to_basin(basin, Point {
                x: x - 1,
                y,
                val: *left.unwrap(),
            }, row, table);
        };
    };

    // right
    if (x + 1) >= 0 {
        let right = row.get(x as usize + 1);
        if right.is_some() {
            recursively_add_to_basin(basin, Point {
                x: x + 1,
                y,
                val: *right.unwrap(),
            }, row, table);
        };
    };

    // up
    if (y - 1) >= 0 {
        let up_row = table.get(y as usize - 1);
        if up_row.is_some() {
            let up = up_row.unwrap().get(x as usize);
            if up.is_some() {
                recursively_add_to_basin(
                    basin,
                    Point {
                        x,
                        y: y - 1,
                        val: *up.unwrap(),
                    },
                    row,
                    table,
                );
            }
        }
    };

    // down
    if (y + 1) >= 0 {
        let down_row = table.get(y as usize + 1);
        if down_row.is_some() {
            let down = down_row.unwrap().get(x as usize);
            if down.is_some() {
                recursively_add_to_basin(
                    basin,
                    Point {
                        x,
                        y: y + 1,
                        val: *down.unwrap(),
                    },
                    row,
                    table,
                );
            }
        }
    };
}

pub fn run_part2(path: &str) -> i32 {
    let content = util::get_file_content(path);
    let table = content
        .lines()
        .map(|l| {
            l.chars()
                .map(|num| num.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut basins: Vec<HashSet<Point>> = Vec::new();

    let mut y = 0;
    for row in &table {
        for (x, num) in row.iter().enumerate() {
            let current_point = Point {
                val: *num,
                x: x as i32,
                y
            };

            if *num == 9 || basins.iter().any(|basin| basin.contains(&current_point)) {
                continue;
            }

            let mut basin = HashSet::new();

            recursively_add_to_basin(&mut basin, current_point, row, &table);

            basins.push(basin);
        }
        y += 1;
    }

    println!("basins {:?}", basins);

    0
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{run_part1, run_part2};

    #[test]
    fn part1_example() {
        assert_eq!(15, run_part1("day09_example.txt"))
    }

    #[test]
    fn part2_example() {
        assert_eq!(1134, run_part2("day09_example.txt"))
    }

    #[test]
    fn part1() {
        assert_eq!(475, run_part1("day09.txt"))
    }

    #[ignore]
    #[test]
    fn part2() {
        assert_eq!(-1, run_part2("day09.txt"))
    }
}
