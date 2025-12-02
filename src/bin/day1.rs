use aoc2025::aoc;

mod part1 {
    use super::*;

    pub fn parse_line(l: &str) -> i64 {
        let (dir, num) = l.split_at(1);

        let sign = match dir {
            "L" => -1,
            "R" => 1,
            _ => panic!("Unexpected char"),
        };

        sign * num.parse::<i64>().unwrap()
    }

    pub fn calculate(input: &str) -> usize {
        let mut pos = 50;
        let mut zeroes = 0;
        const DIAL_SIZE: i64 = 100;

        for i in input.lines().map(parse_line) {
            pos += i;
            pos = pos % DIAL_SIZE;
            zeroes += (pos == 0) as usize;
        }

        zeroes
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day1.txt");
            assert_eq!(calculate(&input), 3);
        }
    }
}
/*
mod part2 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        0
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day1.txt");
            assert_eq!(calculate(&input), 0);
        }
    }
}
*/
fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    // println!("Part 2: {}", part2::calculate(&input));
}
