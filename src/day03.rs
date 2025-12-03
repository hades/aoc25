use std::iter::once_with;

use crate::solver::Solver;

#[derive(Default)]
pub struct Day3Solver {
    input: Vec<Vec<u8>>,
}

fn to_number(digits: &[u8], digit_idxs: &[usize]) -> u64 {
    let mut res = 0u64;
    for i in digit_idxs {
        res = res * 10 + digits[*i] as u64;
    }
    res
}

impl Solver for Day3Solver {
    fn presolve(&mut self, input: &str) {
        self.input = input
            .trim()
            .lines()
            .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
            .collect();
    }

    fn solve_part_one(&mut self) -> String {
        self.input
            .iter()
            .map(|bank| {
                let mut max = u8::MIN;
                for i in 0..bank.len() {
                    for j in (i + 1)..bank.len() {
                        max = max.max(bank[i] * 10 + bank[j]);
                    }
                }
                max as u64
            })
            .sum::<u64>()
            .to_string()
    }

    fn solve_part_two(&mut self) -> String {
        self.input
            .iter()
            .map(|bank| {
                let start_digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
                let mut max = (to_number(bank, &start_digits), start_digits);
                for i in 12..bank.len() {
                    let base_digits = max.1.clone();
                    for replacement in 0..12 {
                        let new_digits: Vec<usize> = base_digits[0..replacement]
                            .iter()
                            .chain(&base_digits[(replacement + 1)..12])
                            .chain(once_with(|| &i))
                            .cloned()
                            .collect();
                        max = max.max((to_number(bank, &new_digits), new_digits));
                    }
                }
                max.0
            })
            .sum::<u64>()
            .to_string()
    }
}

pub fn solver() -> Day3Solver {
    Day3Solver::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example() {
        let example = "987654321111111
811111111111119
234234234234278
818181911112111";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("357", s.solve_part_one());
        assert_eq!("3121910778619", s.solve_part_two());
    }
}
