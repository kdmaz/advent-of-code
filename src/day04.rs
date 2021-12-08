// https://adventofcode.com/2021/day/4

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<i32>>,
}

fn get_drawn_numbers(first_part: &str) -> Vec<i32> {
    first_part
        .trim()
        .split(",")
        .map(|drawn_num| drawn_num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn get_bingo_boards(second_part: &str) -> Vec<Board> {
    let split;

    if cfg!(windows) {
        split = second_part.trim().split("\r\n\r\n")
    } else {
        split = second_part.trim().split("\n\n")
    }

    split
        .map(|board| {
            let to_int = |num: &str| num.parse::<i32>().unwrap();

            let to_number_vec =
                |line: &str| line.split_whitespace().map(to_int).collect::<Vec<i32>>();

            let rows = board.lines().map(to_number_vec).collect::<Vec<Vec<i32>>>();

            Board { rows }
        })
        .collect::<Vec<Board>>()
}

fn get_drawn_numbers_and_bingo_boards(path: &str) -> (Vec<i32>, Vec<Board>) {
    let content = util::get_file_content(path);
    let mut split_content = content.splitn(2, "\n");
    let first_part = split_content.next().unwrap();
    let second_part = split_content.next().unwrap();

    let drawn_numbers = get_drawn_numbers(first_part);
    let bingo_boards = get_bingo_boards(second_part);
    println!("boards: {:?}", bingo_boards);
    (drawn_numbers, bingo_boards)
}

pub fn run_part1(path: &str) -> i32 {
    let (drawn_numbers, bingo_boards) = get_drawn_numbers_and_bingo_boards(path);
    let mut drawn_number_set = HashSet::new();
    let mut last_drawn_num = 0;
    let mut winner = None;

    'draw_number: for drawn_number in drawn_numbers {
        drawn_number_set.insert(drawn_number);
        last_drawn_num = drawn_number;

        for board in &bingo_boards {
            // search rows for match
            for row in &board.rows {
                if row.iter().all(|num| drawn_number_set.contains(num)) {
                    winner = Some(board);
                    break 'draw_number;
                }
            }

            // search columns for match
            for i in 0..board.rows.len() {
                if board.rows.iter().all(|row| {
                    let num = row.get(i).unwrap();
                    drawn_number_set.contains(num)
                }) {
                    winner = Some(board);
                    break 'draw_number;
                }
            }
        }
    }

    let winner = winner.unwrap();

    let unmarked_numbers_sum = winner.rows.iter().flatten().fold(0, |sum, num| {
        if !drawn_number_set.contains(num) {
            sum + num
        } else {
            sum
        }
    });

    last_drawn_num * unmarked_numbers_sum
}

pub fn run_part2(path: &str) -> i32 {
    let (drawn_numbers, bingo_boards) = get_drawn_numbers_and_bingo_boards(path);
    let mut drawn_number_set = HashSet::new();
    let mut winners = HashMap::new();
    let mut winners_order = Vec::new();

    for drawn_number in drawn_numbers {
        drawn_number_set.insert(drawn_number);
        let last_drawn_num = drawn_number;

        for (board_index, board) in bingo_boards.iter().enumerate() {
            // search rows for match
            for row in &board.rows {
                if row.iter().all(|num| drawn_number_set.contains(num)) {
                    if winners.get(&board_index).is_none() {
                        winners.insert(
                            board_index,
                            (last_drawn_num, board, drawn_number_set.clone()),
                        );
                        winners_order.push(board_index);
                    }
                }
            }

            // search columns for match
            for i in 0..board.rows.len() {
                if board.rows.iter().all(|row| {
                    let num = row.get(i).unwrap();
                    drawn_number_set.contains(num)
                }) {
                    if winners.get(&board_index).is_none() {
                        winners.insert(
                            board_index,
                            (last_drawn_num, board, drawn_number_set.clone()),
                        );
                        winners_order.push(board_index);
                    }
                }
            }
        }
    }

    let last_index = winners_order.last().unwrap();
    let (last_drawn_num, winner, drawn_number_set) = winners.get(last_index).unwrap();

    let unmarked_numbers_sum = winner.rows.iter().flatten().fold(0, |sum, num| {
        if !drawn_number_set.contains(num) {
            sum + num
        } else {
            sum
        }
    });

    last_drawn_num * unmarked_numbers_sum
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{run_part1, run_part2};

    #[test]
    fn part1_example() {
        assert_eq!(4512, run_part1("day04_example.txt"));
    }

    #[test]
    fn part2_example() {
        assert_eq!(1924, run_part2("day04_example.txt"));
    }

    #[test]
    fn part1() {
        assert_eq!(58838, run_part1("day04.txt"));
    }

    #[test]
    fn part2() {
        assert_eq!(6256, run_part2("day04.txt"));
    }
}
