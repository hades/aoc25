use std::collections::HashSet;

use crate::solver::Solver;

#[derive(Default)]
pub struct Day2Solver {
    input: Vec<(i64, i64)>,
}

impl Solver for Day2Solver {
    fn presolve(&mut self, input: &str) {
        self.input = input
            .trim()
            .split(",")
            .map(|line| {
                let (from, to) = line.split_once("-").unwrap();
                (from.parse().unwrap(), to.parse().unwrap())
            })
            .collect();
    }

    fn solve_part_one(&mut self) -> String {
        let mut supersum = 0;
        for (from, to) in self.input.clone() {
            for l in 1..10 {
                let pow = 10i64.pow(l);
                let mut lower = pow / 10;
                let mut upper = pow - 1;
                let high_register = from / pow;
                lower = lower.max(high_register);
                if from % pow > lower && lower == high_register {
                    lower += 1;
                }
                let high_register = to / pow;
                upper = upper.min(high_register);
                if to % pow < upper && upper == high_register {
                    upper -= 1;
                }
                if upper < lower {
                    continue;
                }
                let count = (upper - lower) + 1;
                let sum = (lower * pow + lower + upper * pow + upper) * count / 2;
                supersum += sum;
            }
        }
        supersum.to_string()
    }

    fn solve_part_two(&mut self) -> String {
        let mut supersum = 0;
        for (from, to) in self.input.clone() {
            let mut nums = HashSet::new();
            for i in from..=to {
                let s = i.to_string();
                for j in 1..=(s.len() / 2) {
                    if s.matches(&s[..j]).count() * j == s.len() {
                        nums.insert(i);
                    }
                }
            }
            supersum += nums.iter().sum::<i64>();
        }
        supersum.to_string()
    }
}

pub fn solver() -> Day2Solver {
    Day2Solver::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example() {
        let example = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("1227775554", s.solve_part_one());
        assert_eq!("4174379265", s.solve_part_two());
    }

    #[test]
    fn why_cant_you_be_normal() {
        let example = "1052-2547";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("27270", s.solve_part_one());
        assert_eq!("27270", s.solve_part_two());
    }

    #[test]
    fn just_work_plz() {
        let example = "95-115";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("99", s.solve_part_one());
        assert_eq!("210", s.solve_part_two());
    }
}
