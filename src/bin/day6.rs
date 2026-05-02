use std::{fmt::Write, str::FromStr};

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
            s => Err(format!("Unrecognised op: {s:?}"))?,
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

    const fn base(&self) -> u64 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }

    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Op::Add => '+',
            Op::Mul => '*',
        })
    }
}

mod part1 {
    use super::Op;

    pub fn calculate(input: &str) -> u64 {
        let lines: Vec<_> = input.lines().collect();

        let ops: Vec<Op> = lines
            .last()
            .unwrap()
            .split_whitespace()
            .map(str::parse)
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
            assert_eq!(calculate(&input), 4_277_556);
        }
    }
}

mod part2 {
    use super::Op;

    pub fn calculate(input: &str) -> u64 {
        let lines: Vec<_> = input.lines().collect();
        let num_lines = &lines[..lines.len() - 1];
        let ops_line = lines[lines.len() - 1];

        let ops: Vec<Op> = ops_line
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        let lens = {
            let mut lens: Vec<usize> = ops_line
                .as_bytes()
                .split(|c: &u8| *c != b' ')
                .skip(1) // First one will be empty because it comes before the first op
                .map(<[u8]>::len)
                .collect();

            let l = lens.len();
            lens[l - 1] += 1;

            lens
        };

        let mut offset = 0;

        let mut grand_total = 0;
        #[allow(unused_variables)]
        for (col_i, (op, len)) in ops.into_iter().zip(lens).enumerate() {
            let mut col_total = op.base();

            #[cfg(test)]
            println!("Parsing column {col_i}: op={op}, len={len}, base={col_total}");

            for i in 0..len {
                let mut num = 0;
                for line in num_lines {
                    let line = line.as_bytes();
                    if let digit @ b'0'..=b'9' = line[offset + i] {
                        num *= 10;
                        num += u64::from(digit - b'0');
                    }
                }
                #[cfg(test)]
                println!(" i={i}: parsed num={num}");
                col_total = op.apply(col_total, num);
            }
            offset += len + 1;

            #[cfg(test)]
            println!(" total={col_total}");

            grand_total += col_total;
        }

        grand_total
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day6.txt");
            println!("{input}");
            assert_eq!(calculate(&input), 3_263_827);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}
