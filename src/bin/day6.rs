use std::str::FromStr;

use aoc2025::aoc;

enum Op {
    Add,
    Mul,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "+" => Op::Add,
            "*" => Op::Mul,
            s => Err(format!("Unrecognised op: {:?}", s))?,
        })
    }
}

impl Op {
    pub fn reduce(&self, args: impl Iterator<Item = u64>) -> u64 {
        match self {
            Op::Add => args.sum(),
            Op::Mul => args.product(),
        }
    }
}

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> u64 {
        let lines: Vec<_> = input.lines().collect();

        let ops: Vec<Op> = lines
            .last()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .unwrap();

        let nums: Vec<Vec<u64>> = lines[..lines.len() - 1]
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect();

        let arg_count = nums.len();

        ops.iter()
            .enumerate()
            .map(|(i, op)| op.reduce((0..arg_count).map(|j| nums[j][i])))
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day6.txt");
            assert_eq!(calculate(&input), 4277556);
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
            let input = aoc::example::example_string("day6.txt");
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
