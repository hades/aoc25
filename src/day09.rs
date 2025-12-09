use itertools::Itertools;

use crate::solver::Solver;

#[derive(Default)]
pub struct Day8Solver {
    tiles: Vec<(i64, i64)>,
    //crossings: HashMap<i64, Vec<usize>>,
}

impl Day8Solver {
    //fn is_inside((x, y): (i64, i64)) -> bool {
    //    todo!()
    //}
}

impl Solver for Day8Solver {
    fn presolve(&mut self, input: &str) {
        self.tiles = input
            .trim()
            .lines()
            .map(|l| {
                let (x, y) = l.split_once(",").unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();
    }

    fn solve_part_one(&mut self) -> String {
        self.tiles
            .iter()
            .cartesian_product(self.tiles.iter())
            .map(|(a, b)| ((b.0 - a.0 + 1) * (b.1 - a.1 + 1)).abs())
            .max()
            .unwrap()
            .to_string()
    }

    fn solve_part_two(&mut self) -> String {
        todo!()
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
        let example = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("50", s.solve_part_one());
        //assert_eq!("25272", s.solve_part_two());
    }
}
