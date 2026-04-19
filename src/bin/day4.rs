use std::fmt::Write;

use grid::{Grid, GridCell, Point};

pub enum State {
    Empty,
    Paper,
}

impl State {
    #[must_use]
    pub const fn to_char(&self) -> char {
        match self {
            State::Empty => '.',
            State::Paper => '@',
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_char())
    }
}

impl std::str::FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "." => Self::Empty,
            "@" => Self::Paper,
            _ => Err(format!("Unexpected value {s}"))?,
        })
    }
}

mod part1 {
    use super::{Grid, State};

    pub fn calculate(input: &str) -> usize {
        let floor: Grid<Vec<State>> = input.parse().unwrap();
        let counts = Grid::<Vec<u8>>::fill(floor.width(), floor.height(), 0);

        for cell in &floor {
            let point = cell.point();
            match *cell.value() {
                State::Empty => {}
                State::Paper => {
                    for mut neighbour in counts.cell_at_point(&point).unwrap().neighbours() {
                        *neighbour.value_mut() += 1;
                    }
                }
            }
        }

        let mut total = 0;

        for cell in &floor {
            match *cell.value() {
                State::Empty => {}
                State::Paper => {
                    if *counts.cell_at_point(&cell.point()).unwrap().value() < 4 {
                        total += 1;
                    }
                }
            }
        }

        total
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day4.txt");
            assert_eq!(calculate(&input), 13);
        }
    }
}

mod part2 {
    use super::{Grid, GridCell, Point, State};

    fn can_remove(cell: &GridCell<'_, Vec<State>>) -> bool {
        match *cell.value() {
            State::Empty => false,
            State::Paper => {
                let mut count = 0;
                for neighbour in cell.neighbours() {
                    match *neighbour.value() {
                        State::Empty => {}
                        State::Paper => {
                            count += 1;
                        }
                    }
                }

                count < 4
            }
        }
    }

    pub fn calculate(input: &str) -> usize {
        let mut removed = 0;
        let floor: Grid<Vec<State>> = input.parse().unwrap();

        loop {
            let mut to_remove: Vec<Point> = Vec::new();
            for cell in &floor {
                if can_remove(&cell) {
                    to_remove.push(cell.point());
                }
            }

            if to_remove.is_empty() {
                break;
            }
            removed += to_remove.len();

            #[cfg(test)]
            println!("Removing {} rolls", to_remove.len());

            for point in to_remove {
                *floor.cell_at_point(&point).unwrap().value_mut() = State::Empty;
            }
        }

        removed
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day4.txt");
            assert_eq!(calculate(&input), 43);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}
