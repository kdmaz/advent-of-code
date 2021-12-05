#[derive(Debug)]
struct VentLines {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

pub fn run_part1() -> i32 {
    let content = util::get_file_content("day05.txt");

    let vent_lines = content
        .lines()
        .map(|line| {
            let mut line = line.splitn(2, ",");
            let x1 = line.next().unwrap().trim().parse::<i32>().unwrap();
            let mut line = line.next().unwrap().splitn(2, "->");
            let y1 = line.next().unwrap().trim().parse::<i32>().unwrap();
            let mut line = line.next().unwrap().splitn(2, ",");
            let x2 = line.next().unwrap().trim().parse::<i32>().unwrap();
            let y2 = line.next().unwrap().trim().parse::<i32>().unwrap();

            VentLines { x1, y1, x2, y2 }
        })
        .collect::<Vec<VentLines>>();

    for l in vent_lines {
        println!("{:?}", l);
    }
    0
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::run_part1;

    #[test]
    fn part1_correct() {
        assert_eq!(-1, run_part1())
    }

    // #[test]
    // fn part2_correct() {}
}
