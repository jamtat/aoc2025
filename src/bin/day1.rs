use aoc2025::aoc;

const START: i64 = 50;
const DIAL_SIZE: i64 = 100;

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
        let mut pos = START;
        let mut zeroes = 0;

        for i in input.lines().map(parse_line) {
            pos += i;
            pos = pos.rem_euclid(DIAL_SIZE);
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

mod part2 {

    use crate::part1::parse_line;

    use super::*;

    pub fn calculate(input: &str) -> usize {
        let mut pos = START;
        let mut zeroes = 0;

        for l in input.lines() {
            let i = parse_line(l);
            if i == 0 {
                continue;
            }
            let mut points: usize = 0;
            let full_rotations = (i / DIAL_SIZE).abs() as usize;
            points += full_rotations;

            let rem = i % DIAL_SIZE;

            if pos != 0 && rem != 0 {
                let newpos = pos + rem;

                let rem_cross = newpos >= 100 || newpos <= 0;
                if rem_cross {
                    points += 1;
                }
            }

            pos += i;
            pos = pos.rem_euclid(DIAL_SIZE);

            zeroes += points;

            #[cfg(test)]
            if points > 0 {
                use colored::Colorize;

                println!(
                    "The dial is rotated {l} to point at {pos}; during the rotation it points at zero {} time{} ({full_rotations} full rotations)",
                    (&format!("{points}")).bold(),
                    if points == 1 { ' ' } else { 's' },
                );
            } else {
                println!("The dial is rotated {l} to point at {pos}.");
            }
        }

        zeroes
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day1.txt");
            assert_eq!(calculate(&input), 6);
        }

        #[test]
        fn test_more() {
            assert_eq!(calculate("R1000"), 10);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input)); // 5978
}
