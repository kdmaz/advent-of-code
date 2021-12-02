use std::{fs, ops::Add};

struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn get_position_change(command: &str, units: i32) -> Position {
    match command {
        "forward" => Position { x: units, y: 0 },
        "up" => Position { x: 0, y: -units },
        "down" => Position { x: 0, y: units },
        _ => panic!("received unknown command"),
    }
}

pub fn run_part1() -> i32 {
    let filename = "day02.txt";
    let content =
        fs::read_to_string(filename).expect(&format!("Could not read file from {}", filename));

    let Position { x, y } = content
        .lines()
        .fold(Position { x: 0, y: 0 }, |position, current| {
            let mut line = current.split_whitespace();
            let command = line.next().expect("cannot parse command from line");
            let units = line
                .next()
                .expect("cannot parse unit from line")
                .parse::<i32>()
                .expect("cannot parse unit into i32");
            position + get_position_change(command, units)
        });

    x * y
}

#[cfg(test)]
mod tests {
    use crate::day02::run_part1;

    #[test]
    fn part1_correct() {
        assert_eq!(2102357, run_part1())
    }
}
