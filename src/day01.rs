use crate::solver::Solver;

#[derive(Default)]
pub struct Day1Solver {
    input: Vec<i64>,
}

impl Solver for Day1Solver {
    fn presolve(&mut self, input: &str) {
        self.input = input
            .trim()
            .split("\n")
            .map(|line| {
                if let Some(n) = line.strip_prefix('L') {
                    -n.parse::<i64>().unwrap()
                } else if let Some(n) = line.strip_prefix('R') {
                    n.parse().unwrap()
                } else {
                    panic!("what: {line}");
                }
            })
            .collect();
    }

    fn solve_part_one(&mut self) -> String {
        let mut p = 50;
        let mut count = 0;
        for n in self.input.clone() {
            p += n;
            if p % 100 == 0 {
                count += 1;
            }
        }
        count.to_string()
    }

    fn solve_part_two(&mut self) -> String {
        let mut count = 0;
        let mut p = 1000000000050;
        for i in self.input.clone() {
            if p % 100 == 0 {
                count += (i / 100).abs();
            } else {
                count += ((p + i) / 100 - p / 100).abs();
                if i < 0 && (p + i) % 100 == 0 {
                    count += 1;
                }
            }
            p += i;
        }
        count.to_string()
    }
}

pub fn solver() -> Day1Solver {
    Day1Solver::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example() {
        let example = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("3", s.solve_part_one());
        assert_eq!("6", s.solve_part_two());
    }
}
