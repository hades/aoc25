use itertools::Itertools;

use crate::solver::Solver;

#[derive(Default)]
pub struct Day8Solver {
    boxes: Vec<(i64, i64, i64)>,
    pairs_by_distance: Vec<(usize, usize)>,
}

impl Day8Solver {
    fn distance(&self, i: usize, j: usize) -> f64 {
        let dx = (self.boxes[i].0 - self.boxes[j].0) as f64;
        let dy = (self.boxes[i].1 - self.boxes[j].1) as f64;
        let dz = (self.boxes[i].2 - self.boxes[j].2) as f64;
        dx * dx + dy * dy + dz * dz
    }

    fn solve_part_one_limit(&mut self, limit: usize) -> String {
        let mut circuit_id = (0..self.boxes.len()).collect::<Vec<_>>();
        for (a, b) in self.pairs_by_distance.iter().take(limit) {
            let a_circuit = circuit_id[*a];
            let b_circuit = circuit_id[*b];
            if a_circuit == b_circuit {
                continue;
            }
            circuit_id.iter_mut().for_each(|old_id| {
                if *old_id == b_circuit {
                    *old_id = a_circuit
                }
            });
        }
        let mut counts = circuit_id.iter().counts().into_values().collect::<Vec<_>>();
        counts.sort();
        counts.reverse();
        counts.into_iter().take(3).product::<usize>().to_string()
    }
}

impl Solver for Day8Solver {
    fn presolve(&mut self, input: &str) {
        self.boxes = input
            .lines()
            .map(|line| {
                let mut split = line.split(",");
                let x = split.next().unwrap().parse().unwrap();
                let y = split.next().unwrap().parse().unwrap();
                let z = split.next().unwrap().parse().unwrap();
                (x, y, z)
            })
            .collect();
        let mut pairs = (0..self.boxes.len())
            .flat_map(|i| (0..i).map(move |j| (i, j)))
            .collect::<Vec<_>>();
        pairs.sort_by(|&a, &b| self.distance(a.0, a.1).total_cmp(&self.distance(b.0, b.1)));
        self.pairs_by_distance = pairs;
    }

    fn solve_part_one(&mut self) -> String {
        self.solve_part_one_limit(1000)
    }

    fn solve_part_two(&mut self) -> String {
        let mut circuit_id = (0..self.boxes.len()).collect::<Vec<_>>();
        for (a, b) in self.pairs_by_distance.iter() {
            let a_circuit = circuit_id[*a];
            let b_circuit = circuit_id[*b];
            if a_circuit == b_circuit {
                continue;
            }
            circuit_id.iter_mut().for_each(|old_id| {
                if *old_id == b_circuit {
                    *old_id = a_circuit
                }
            });
            if circuit_id.iter().all(|&x| x == a_circuit) {
                return (self.boxes[*a].0 * self.boxes[*b].0).to_string();
            }
        }
        panic!("did not complete circuit");
    }
}

pub fn solver() -> Day8Solver {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example() {
        let example = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("40", s.solve_part_one_limit(10));
        assert_eq!("25272", s.solve_part_two());
    }
}
