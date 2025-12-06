use interval::prelude::{Cardinality, Contains, ToIntervalSet};
use interval::{IntervalSet, prelude::Empty};

use crate::solver::Solver;

pub struct Day5Solver {
    fresh: IntervalSet<i64>,
    available: Vec<i64>,
}

impl Solver for Day5Solver {
    fn presolve(&mut self, input: &str) {
        let (fresh, available) = input.split_once("\n\n").unwrap();
        let fresh = fresh
            .lines()
            .map(|line| {
                let (from, to) = line.split_once("-").unwrap();
                (from.parse().unwrap(), to.parse().unwrap())
            })
            .collect::<Vec<_>>();
        self.fresh = fresh.to_interval_set();
        self.available = available
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
    }

    fn solve_part_one(&mut self) -> String {
        self.available
            .iter()
            .filter(|&i| self.fresh.contains(i))
            .count()
            .to_string()
    }

    fn solve_part_two(&mut self) -> String {
        self.fresh.size().to_string()
    }
}

pub fn solver() -> Day5Solver {
    Day5Solver {
        fresh: IntervalSet::empty(),
        available: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example() {
        let example = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("3", s.solve_part_one());
        assert_eq!("14", s.solve_part_two());
    }
}
