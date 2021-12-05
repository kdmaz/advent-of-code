use std::{
    collections::{HashMap, HashSet},
    str::Split,
};

struct Board {
    rows: Vec<Vec<i32>>,
}

fn get_drawn_numbers(split_content: &mut Split<&str>) -> Vec<i32> {
    split_content
        .next()
        .expect("could not get drawn numbers")
        .split(",")
        .map(|drawn_num| {
            drawn_num
                .parse::<i32>()
                .expect(&format!("could not parse ({}) into integer", drawn_num))
        })
        .collect::<Vec<i32>>()
}

fn get_bingo_boards(split_content: &mut Split<&str>) -> Vec<Board> {
    split_content
        .map(|board| {
            let to_int = |num: &str| {
                num.parse::<i32>()
                    .expect(&format!("could not parse ({}) into integer", num))
            };

            let to_number_vec =
                |line: &str| line.split_whitespace().map(to_int).collect::<Vec<i32>>();

            let rows = board.lines().map(to_number_vec).collect::<Vec<Vec<i32>>>();

            Board { rows }
        })
        .collect::<Vec<Board>>()
}

pub fn run_part1() -> i32 {
    let content = util::get_file_content("day04.txt");
    let mut split_content = content.split("\r\n\r\n");

    let drawn_numbers = get_drawn_numbers(&mut split_content);
    let bingo_boards = get_bingo_boards(&mut split_content);

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
                    let num = row
                        .get(i)
                        .expect(&format!("could not find column ({}) on row", i));
                    drawn_number_set.contains(num)
                }) {
                    winner = Some(board);
                    break 'draw_number;
                }
            }
        }
    }

    let winner = winner.expect("did not find a winner");

    let unmarked_numbers_sum = winner.rows.iter().flatten().fold(0, |sum, num| {
        if !drawn_number_set.contains(num) {
            sum + num
        } else {
            sum
        }
    });

    last_drawn_num * unmarked_numbers_sum
}

pub fn run_part2() -> i32 {
    let content = util::get_file_content("day04.txt");
    let mut split_content = content.split("\r\n\r\n");

    let drawn_numbers = get_drawn_numbers(&mut split_content);
    let bingo_boards = get_bingo_boards(&mut split_content);

    let mut drawn_number_set = HashSet::new();
    let mut winners = HashMap::new();
    let mut winners_order = Vec::new();

    for drawn_number in drawn_numbers {
        drawn_number_set.insert(drawn_number);
        let last_drawn_num = drawn_number;

        // remove won boards from list
        // break if one bingo_boards.len() left
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
                    let num = row
                        .get(i)
                        .expect(&format!("could not find column ({}) on row", i));
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

    let last_index = winners_order.last().expect("could not get last index");
    let (last_drawn_num, winner, drawn_number_set) = winners
        .get(last_index)
        .expect(&format!("could not find last index ({})", last_index));

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
    fn part1_correct() {
        assert_eq!(58838, run_part1());
    }

    #[test]
    fn part2_correct() {
        assert_eq!(6256, run_part2());
    }
}
