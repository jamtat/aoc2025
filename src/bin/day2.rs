use std::ops::RangeInclusive;

use aoc2025::aoc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range(usize, usize);

impl Range {
    pub fn iter(&self) -> RangeInclusive<usize> {
        self.0..=self.1
    }
}

fn parse_pair(s: &str) -> Range {
    let (l, r) = s.split_once('-').unwrap();
    Range(l.parse().unwrap(), r.parse().unwrap())
}

pub fn parse_input(input: &str) -> Vec<Range> {
    input.split(',').map(parse_pair).collect()
}

mod part1 {
    use super::*;

    fn is_invalid(i: usize) -> bool {
        if i < 10 {
            return false;
        }
        let digits = i.ilog10() + 1;

        if (digits % 2) == 1 {
            return false;
        }

        let splitter = 10usize.pow(digits / 2);

        let first_half = i / splitter;
        let last_half = i.rem_euclid(splitter);

        first_half == last_half
    }

    pub fn calculate(input: &str) -> usize {
        parse_input(input)
            .iter()
            .map(Range::iter)
            .flatten()
            .filter(|i| is_invalid(*i))
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day2.txt");
            assert_eq!(calculate(&input), 1227775554);
        }

        #[test]
        fn test_is_invalid() {
            assert!(!is_invalid(0));
            assert!(!is_invalid(1));
            assert!(!is_invalid(10));
            assert!(!is_invalid(123321));
            assert!(!is_invalid(1001));

            assert!(is_invalid(55));
            assert!(is_invalid(1010));
            assert!(is_invalid(123123));
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
            let input = aoc::example::example_string("day2.txt");
            assert_eq!(calculate(&input), 0);
        }
    }
}
*/
fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input)); // 35367539282
    // println!("Part 2: {}", part2::calculate(&input));
}
