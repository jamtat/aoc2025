use aoc2025::aoc;

pub struct Bank(Vec<u8>);

impl Bank {
    pub fn max_joltage(&self) -> u8 {
        let mut max = 0;

        for (i, bat1) in self.0[..self.0.len() - 1].iter().enumerate() {
            for bat2 in self.0[i + 1..].iter() {
                let joltage = bat1 * 10 + bat2;
                max = max.max(joltage);
            }
        }

        max
    }
}

impl std::str::FromStr for Bank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Bank(s.bytes().map(|b| b - b'0').collect()))
    }
}

fn parse_input(input: &str) -> Vec<Bank> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        parse_input(input)
            .iter()
            .map(|bank| bank.max_joltage() as usize)
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day3.txt");
            assert_eq!(calculate(&input), 357);
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
            let input = aoc::example::example_string("day3.txt");
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
