pub fn run_part1() -> i32 {
    3
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
