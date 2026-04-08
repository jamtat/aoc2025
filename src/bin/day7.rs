use aoc2025::aoc;
use std::{cell::RefCell, collections::HashSet, ops::Deref};

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let lines: Vec<_> = input.lines().collect();

        let splitterss = input
            .lines()
            .map(|line| {
                line.bytes()
                    .enumerate()
                    .filter_map(|(i, b)| (b == b'^').then_some(i))
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        // println!("{splitterss:#?}");

        let mut beams: Vec<RefCell<HashSet<usize>>> = Vec::new();
        beams.resize_with(splitterss.len(), Default::default);

        beams[0].get_mut().insert(lines[0].find('S').unwrap());

        let mut total_splits = 0;
        for (i, splitters) in splitterss.iter().enumerate().skip(1) {
            let last_beams = &beams[i - 1];
            let these_beams = &beams[i];
            for beam in last_beams.borrow().deref() {
                let mut these_beams = these_beams.borrow_mut();
                if splitters.contains(beam) {
                    total_splits += 1;
                    these_beams.insert(beam - 1);
                    these_beams.insert(beam + 1);
                } else {
                    these_beams.insert(*beam);
                }
            }
        }

        total_splits
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day7.txt");
            assert_eq!(calculate(&input), 21);
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
            let input = aoc::example::example_string("day7.txt");
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
