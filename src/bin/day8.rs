use itertools::Itertools;
use quant::npoint::NPoint;

type Loc = NPoint<f64, 3>;

fn parse_input(s: &str) -> Vec<Loc> {
    s.lines()
        .map(|l| {
            Loc::new(
                l.split(',')
                    .map(|n| n.parse::<f64>().unwrap())
                    .collect_array::<3>()
                    .unwrap(),
            )
        })
        .collect()
}

mod part1 {
    use std::collections::{HashMap, VecDeque};

    use super::parse_input;

    pub fn calculate(input: &str, cnn_count: usize) -> usize {
        let locs = parse_input(input);

        #[allow(unused)]
        let mut connections: usize = 0;
        let mut circuits: Vec<usize> = (0..locs.len()).collect();
        let mut distances: Vec<((usize, usize), f64)> = Vec::with_capacity(locs.len());

        for (i, a) in locs.iter().enumerate() {
            for (j, b) in locs.iter().enumerate().skip(i + 1) {
                distances.push(((i, j), a.distance(b)));
            }
        }

        distances.sort_by(|(_, da), (_, db)| da.partial_cmp(db).unwrap());
        let mut distances = VecDeque::from(distances);

        for _ in 0..cnn_count {
            let ((a, b), _) = distances.pop_front().unwrap();

            let a_circuit = circuits[a];
            let b_circuit = circuits[b];

            if a_circuit == b_circuit {
                #[cfg(test)]
                println!(
                    "Already conncted {} to {} (circuit {})",
                    locs[a], locs[b], a_circuit
                );
                continue;
            }

            // Arbitrarily set all of b_circuit to a_circuit
            for id in &mut circuits {
                if *id == b_circuit {
                    *id = a_circuit;
                }
            }

            #[cfg(test)]
            println!(
                "Connected {} to {} (circuit {})",
                locs[a], locs[b], a_circuit
            );

            #[allow(unused)]
            {
                connections += 1;
            }
        }

        let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();

        for circuit_id in circuits {
            *circuit_sizes.entry(circuit_id).or_default() += 1;
        }

        #[cfg(test)]
        println!("{circuit_sizes:?}");

        let mut circuit_sizes: Vec<_> = circuit_sizes.values().copied().collect();
        circuit_sizes.sort_unstable();
        #[cfg(test)]
        {
            println!("{circuit_sizes:?}");
            println!("Made {connections} connections");
        }

        circuit_sizes.into_iter().rev().take(3).product()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day8.txt");
            assert_eq!(calculate(&input, 10), 40);
        }
    }
}

mod part2 {
    use std::collections::{HashMap, VecDeque};

    use super::parse_input;

    pub fn calculate(input: &str) -> u64 {
        let locs = parse_input(input);

        let mut circuits: Vec<usize> = (0..locs.len()).collect();
        let mut distances: Vec<((usize, usize), f64)> = Vec::with_capacity(locs.len());
        let mut circuit_sizes: HashMap<usize, usize> = (0..locs.len()).map(|i| (i, 1)).collect();

        for (i, a) in locs.iter().enumerate() {
            for (j, b) in locs.iter().enumerate().skip(i + 1) {
                distances.push(((i, j), a.distance(b)));
            }
        }

        distances.sort_by(|(_, da), (_, db)| da.partial_cmp(db).unwrap());
        let mut distances = VecDeque::from(distances);

        loop {
            let ((a, b), _) = distances.pop_front().unwrap();

            let a_circuit = circuits[a];
            let b_circuit = circuits[b];

            if a_circuit == b_circuit {
                #[cfg(test)]
                println!(
                    "Already conncted {} to {} (circuit {})",
                    locs[a], locs[b], a_circuit
                );
                continue;
            }

            // Arbitrarily set all of b_circuit to a_circuit
            for id in &mut circuits {
                if *id == b_circuit {
                    *id = a_circuit;
                }
            }

            #[cfg(test)]
            println!(
                "Connected {} to {} (circuit {})",
                locs[a], locs[b], a_circuit
            );

            let b_size = circuit_sizes.get_mut(&b_circuit).unwrap();
            let to_add = *b_size;
            *b_size = 0;

            let a_size = circuit_sizes.get_mut(&a_circuit).unwrap();
            *a_size += to_add;

            if *a_size == locs.len() {
                let product = *locs[a].get(0) * *locs[b].get(0);
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                return product as u64;
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day8.txt");
            assert_eq!(calculate(&input), 25272);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input, 1000));
    println!("Part 2: {}", part2::calculate(&input));
}
