use double_buffer::DoubleBuffer;
use std::collections::HashSet;

fn find_coords(input: &str, b: u8) -> impl Iterator<Item = (usize, usize)> {
    input.lines().enumerate().flat_map(move |(y, line)| {
        line.bytes()
            .enumerate()
            .filter_map(move |(x, c)| (b == c).then_some((x, y)))
    })
}

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let rows = input.lines().count();
        let splitters = find_coords(input, b'^').collect::<HashSet<_>>();
        let start = find_coords(input, b'S').next().unwrap().0;

        let mut total_splits = 0;

        let beams = DoubleBuffer::from(HashSet::from([start]));
        let last_beams = beams.left();
        let these_beams = beams.right();

        for y in 0..rows {
            beams.swap();
            let mut these_beams = these_beams.borrow_mut();
            these_beams.clear();

            for &beam in last_beams.borrow().iter() {
                if splitters.contains(&(beam, y)) {
                    total_splits += 1;
                    these_beams.insert(beam - 1);
                    these_beams.insert(beam + 1);
                } else {
                    these_beams.insert(beam);
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

mod part2 {
    use std::collections::HashMap;

    use super::*;

    #[allow(unused_assignments, unused_variables)]
    pub fn calculate(input: &str) -> usize {
        let rows = input.lines().count();
        let splitters = find_coords(input, b'^').collect::<HashSet<_>>();
        let start = find_coords(input, b'S').next().unwrap().0;

        let beams = DoubleBuffer::from(HashMap::from([(start, 1)]));
        let last_beams = beams.left();
        let these_beams = beams.right();

        #[cfg(test)]
        println!("start: {these_beams:?}");

        for y in 0..rows {
            beams.swap();
            let mut these_beams = these_beams.borrow_mut();
            these_beams.clear();

            for (&beam, &count) in last_beams.borrow().iter() {
                if count == 0 {
                    continue;
                }
                if splitters.contains(&(beam, y)) {
                    *these_beams.entry(beam - 1).or_default() += count;
                    *these_beams.entry(beam + 1).or_default() += count;
                } else {
                    *these_beams.entry(beam).or_default() += count;
                }
            }

            #[cfg(test)]
            println!("row {y}: {these_beams:#?}");
        }

        these_beams.borrow().values().sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day7.txt");
            assert_eq!(calculate(&input), 40);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}
