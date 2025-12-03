use aoc2025::aoc;

fn max_concat_digits(xs: &[usize], count: u32) -> usize {
    if count == 0 {
        return 0;
    }

    if (count as usize) > xs.len() {
        panic!("Items not long enough to find max concat digits");
    }

    let mut max_digit = 0;
    let mut max_ptr = 0;
    let last_item_allowed = xs.len() - (count as usize);

    for (i, &x) in xs[..=last_item_allowed].iter().enumerate() {
        if x > max_digit {
            max_ptr = i;
            max_digit = x;
            if x == 9 {
                break;
            }
        }
    }

    10usize.pow(count - 1) * max_digit + max_concat_digits(&xs[max_ptr + 1..], count - 1)
}

pub struct Bank(Vec<usize>);

impl std::ops::Deref for Bank {
    type Target = [usize];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for Bank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Bank(s.bytes().map(|b| (b - b'0') as usize).collect()))
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
            .map(|bank| max_concat_digits(bank, 2))
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

mod part2 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        parse_input(input)
            .iter()
            .map(|bank| max_concat_digits(bank, 12))
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day3.txt");
            assert_eq!(calculate(&input), 3121910778619);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}
