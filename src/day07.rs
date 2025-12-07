use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use crate::solver::Solver;

#[derive(Default)]
pub struct Day5Solver {
    start: (usize, usize),
    splitters: Vec<HashSet<usize>>,
}

impl Solver for Day5Solver {
    fn presolve(&mut self, input: &str) {
        let mut start = None;
        let mut splitters = vec![];
        for (i, line) in input.trim().lines().enumerate() {
            splitters.push(HashSet::new());
            for (j, ch) in line.chars().enumerate() {
                match ch {
                    'S' => {
                        start = Some((i, j));
                    }
                    '^' => {
                        splitters.last_mut().unwrap().insert(j);
                    }
                    _ => {}
                }
            }
        }
        self.start = start.unwrap();
        self.splitters = splitters;
    }

    fn solve_part_one(&mut self) -> String {
        let mut beams = once(self.start.1).collect::<HashSet<_>>();
        let mut count = 0;
        for level in (self.start.0 + 1)..(self.splitters.len()) {
            let mut next_beams = HashSet::new();
            for beam_j in beams.drain() {
                if !self.splitters[level].contains(&beam_j) {
                    next_beams.insert(beam_j);
                    continue;
                }
                assert!(!self.splitters[level].contains(&(beam_j - 1)));
                assert!(!self.splitters[level].contains(&(beam_j + 1)));
                next_beams.insert(beam_j - 1);
                next_beams.insert(beam_j + 1);
                count += 1;
            }
            beams = next_beams;
        }
        count.to_string()
    }

    fn solve_part_two(&mut self) -> String {
        let mut beams = once((self.start.1, 1)).collect::<HashMap<_, _>>();
        for level in (self.start.0 + 1)..(self.splitters.len()) {
            let mut next_beams = HashMap::new();
            for (beam_j, count) in beams.drain() {
                if !self.splitters[level].contains(&beam_j) {
                    *next_beams.entry(beam_j).or_insert(0) += count;
                    continue;
                }
                assert!(!self.splitters[level].contains(&(beam_j - 1)));
                assert!(!self.splitters[level].contains(&(beam_j + 1)));
                *next_beams.entry(beam_j - 1).or_insert(0) += count;
                *next_beams.entry(beam_j + 1).or_insert(0) += count;
            }
            beams = next_beams;
        }
        beams.into_values().sum::<i64>().to_string()
    }
}

pub fn solver() -> Day5Solver {
    Day5Solver::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example() {
        let example = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("21", s.solve_part_one());
        assert_eq!("40", s.solve_part_two());
    }
}
