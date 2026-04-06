use std::str::FromStr;

use aoc2025::aoc;

struct Fresh {
    start: u64,
    end: u64,
}

impl Fresh {
    pub fn is_fresh(&self, ingredient: u64) -> bool {
        ingredient >= self.start && ingredient <= self.end
    }
}

impl FromStr for Fresh {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or("Could not parse fresh range")?;

        Ok(Self {
            start: start
                .parse()
                .map_err(|_| format!("Could not parse start: {:#}", start))?,
            end: end
                .parse()
                .map_err(|_| format!("Could not parse end: {:#}", end))?,
        })
    }
}

struct Input {
    fresh_ranges: Vec<Fresh>,
    ingredients: Vec<u64>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (fresh_ranges, ingredients) = s.split_once("\n\n").ok_or("Could not parse input")?;

        let fresh_ranges = fresh_ranges
            .lines()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;

        let ingredients = ingredients
            .lines()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Could not parse ingredients")?;

        Ok(Self {
            fresh_ranges,
            ingredients,
        })
    }
}

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let input: Input = input.parse().unwrap();
        let mut count = 0;

        for ingredient in input.ingredients {
            for range in &input.fresh_ranges {
                if range.is_fresh(ingredient) {
                    count += 1;
                    break;
                }
            }
        }

        count
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day5.txt");
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
            let input = aoc::example::example_string("day5.txt");
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
