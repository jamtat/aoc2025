#!/usr/bin/env bash

set -euo pipefail

DAY_NUM="$1"

mkdir -p input/ src/bin/examples/
touch input/day$DAY_NUM.txt examples/day$DAY_NUM.txt

cat << EOF > src/bin/day$DAY_NUM.rs
use aoc;

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        0
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day${DAY_NUM}.txt");
            assert_eq!(calculate(&input), 0);
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
            let input = aoc::example::example_string("day${DAY_NUM}.txt");
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
EOF


code input/day$DAY_NUM.txt src/bin/day$DAY_NUM.rs examples/day$DAY_NUM.txt