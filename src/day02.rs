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

fn get_file_content() -> String {
    let filename = "day02.txt";
    fs::read_to_string(filename).expect(&format!("Could not read file from {}", filename))
}

fn get_command_and_units(line: &str) -> (&str, i32) {
    let mut line = line.split_whitespace();
    let command = line.next().expect("cannot parse command from line");
    let units = line
        .next()
        .expect("cannot parse unit from line")
        .parse::<i32>()
        .expect("cannot parse unit into i32");
    (command, units)
}

pub fn run_part1() -> i32 {
    let content = get_file_content();
    let Position { x, y } = content
        .lines()
        .fold(Position { x: 0, y: 0 }, |position, current| {
            let (command, units) = get_command_and_units(current);
            position + get_position_change(command, units)
        });

    x * y
}

struct ModifiedPosition {
    x: i32,
    y: i32,
    aim: i32
}

impl Add for ModifiedPosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ModifiedPosition { x: self.x + rhs.x, y: self.y + rhs.y, aim: self.aim + rhs.aim }
    }
}

fn get_modified_position_change(command: &str, units: i32, current_aim: i32) -> ModifiedPosition {
    match command {
        "forward" => ModifiedPosition { x: units, y: current_aim * units, aim: 0 },
        "up" => ModifiedPosition { x: 0, y: 0, aim: -units },
        "down" => ModifiedPosition { x: 0, y: 0, aim: units },
        _ => panic!("received unknown command"),
    }
}

pub fn run_part2() -> i32 {
    let content = get_file_content();
    let ModifiedPosition { x, y, .. } = content.lines().fold(ModifiedPosition { x: 0, y: 0, aim: 0 }, |position, current| {
        let (command, units) = get_command_and_units(current);
        let current_aim = position.aim;
        position + get_modified_position_change(command, units, current_aim)
    });

    x * y
}

#[cfg(test)]
mod tests {
    use crate::day02::{run_part1, run_part2};

    #[test]
    fn part1_correct() {
        assert_eq!(2102357, run_part1());
    }

    #[test]
    fn part2_correct() {
        assert_eq!(2101031224, run_part2());
    }
}
