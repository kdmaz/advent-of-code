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

    for board in bingo_boards {
        println!("board");
        for nums in board.rows {
            for num in nums {
                print!("{} ", num);
            }
            println!();
        }
        println!();
    }

    0
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::run_part1;

    #[test]
    fn part1_correct() {
        assert_eq!(4512, run_part1());
    }

    #[test]
    fn part2_correct() {}
}
