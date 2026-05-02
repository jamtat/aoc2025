use quant::npoint::NPoint;

struct Point(NPoint<u64, 2>);

impl Point {
    pub fn new(x: u64, y: u64) -> Self {
        Self(NPoint::new([x, y]))
    }

    pub fn x(&self) -> u64 {
        *self.0.get(0)
    }

    pub fn y(&self) -> u64 {
        *self.0.get(1)
    }
}

impl std::str::FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(())?;
        Ok(Self::new(
            x.parse().map_err(|_| ())?,
            y.parse().map_err(|_| ())?,
        ))
    }
}

fn parse_input(s: &str) -> Vec<Point> {
    s.lines().map(|l| l.parse().unwrap()).collect()
}

fn rectangle(a: &Point, b: &Point) -> u64 {
    let x = a.x().abs_diff(b.x()) + 1;
    let y = a.y().abs_diff(b.y()) + 1;

    x * y
}

mod part1 {
    use crate::rectangle;

    use super::parse_input;

    pub fn calculate(input: &str) -> u64 {
        let points = parse_input(input);

        points
            .iter()
            .enumerate()
            .flat_map(|(i, a)| points.iter().skip(i + 1).map(|b| rectangle(a, b)))
            .max()
            .unwrap_or(0)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day9.txt");
            assert_eq!(calculate(&input), 50);
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
            let input = aoc::example::example_string("day9.txt");
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
