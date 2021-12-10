// https://adventofcode.com/2021/day/9

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

#[derive(PartialEq, Eq, Debug, Clone)]
struct Point {
    val: i32,
    x: i32,
    y: i32,
}

pub fn run_part2(path: &str) -> i32 {
    let content = util::get_file_content(path);
    let nums_by_line = content
        .lines()
        .map(|l| {
            l.chars()
                .map(|num| num.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut basins: Vec<Vec<Point>> = Vec::new();

    let mut y = 0;
    for nums in &nums_by_line {
        for (i, num) in nums.iter().enumerate() {
            let left = if (i as i32 - 1) < 0 {
                None
            } else {
                let val = nums.get(i - 1);
                if val.is_some() {
                    Some(Point {
                        x: i as i32 - 1,
                        y,
                        val: *val.unwrap(),
                    })
                } else {
                    None
                }
            };

            let right = if (i as i32 + 1) < 0 {
                None
            } else {
                let val = nums.get(i + 1);
                if val.is_some() {
                    Some(Point {
                        x: i as i32 + 1,
                        y,
                        val: *val.unwrap(),
                    })
                } else {
                    None
                }
            };

            let up = if (y as i32 - 1) < 0 {
                None
            } else {
                match nums_by_line.get(y as usize - 1) {
                    Some(row) => {
                        let val = row.get(i);
                        if val.is_some() {
                            Some(Point {
                                x: i as i32,
                                y: y as i32 - 1,
                                val: *val.unwrap(),
                            })
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            };

            let down = if (y as i32 + 1) < 0 {
                None
            } else {
                match nums_by_line.get(y as usize + 1) {
                    Some(row) => {
                        let val = row.get(i);
                        if val.is_some() {
                            Some(Point {
                                x: i as i32,
                                y: y as i32 + 1,
                                val: *val.unwrap(),
                            })
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            };

            if num != &9 {
                let mut basin_index = None;
                let mut index: usize = 0;

                'basin: for basin in &basins {
                    for point in basin {
                        if left.is_some() && (point == &left.unwrap()) {
                            basin_index = Some(index);
                            break 'basin;
                        }

                        // right

                        // up

                        // down
                    }
                }

                if basin_index.is_some() {
                    // let mut basin = basins.get(basin_index.unwrap()).unwrap();
                    // basin.push(Point {
                    //     val: *num,
                    //     x: i as i32,
                    //     y,
                    // });
                }
            }
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

    #[ignore]
    #[test]
    fn part2_example() {
        assert_eq!(-1, run_part2("day09_example.txt"))
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
