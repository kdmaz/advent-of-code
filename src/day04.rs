use std::collections::HashSet;

struct Board {
    rows: Vec<Vec<i32>>,
}

pub fn run_part1() -> i32 {
    let content = util::get_file_content("day04.txt");
    let mut split_content = content.split("\r\n\r\n");

    let drawn_numbers = split_content
        .next()
        .expect("could not get drawn numbers")
        .split(",")
        .map(|drawn_num| {
            drawn_num
                .parse::<i32>()
                .expect(&format!("could not parse ({}) into integer", drawn_num))
        })
        .collect::<Vec<i32>>();

    let bingo_boards = split_content
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
        .collect::<Vec<Board>>();

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
                    let x = row
                        .get(i)
                        .expect(&format!("could not find column ({}) on row", i));
                    drawn_number_set.contains(x)
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

fn main() {}

#[cfg(test)]
mod tests {
    use crate::run_part1;

    #[test]
    fn part1_correct() {
        assert_eq!(58838, run_part1());
    }

    #[test]
    fn part2_correct() {}
}
