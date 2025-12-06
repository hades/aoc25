use regex::Regex;

use crate::solver::Solver;

#[derive(Default)]
pub struct Day6Solver {
    input: String,
}

impl Solver for Day6Solver {
    fn presolve(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn solve_part_one(&mut self) -> String {
        let mut lines = self.input.lines().peekable();
        let number_re = Regex::new(r"\d+").unwrap();
        let mut values = vec![];
        while lines
            .peek()
            .unwrap()
            .trim()
            .chars()
            .next()
            .unwrap()
            .is_ascii_digit()
        {
            values.push(
                number_re
                    .find_iter(lines.next().unwrap())
                    .map(|n| n.as_str().parse().unwrap())
                    .collect::<Vec<i64>>(),
            );
        }
        let op_re = Regex::new(r"\S").unwrap();
        let ops = op_re
            .find_iter(lines.next().unwrap())
            .map(|n| n.as_str().chars().next().unwrap())
            .collect::<Vec<_>>();
        ops.iter()
            .enumerate()
            .map(|(i, op)| match *op {
                '*' => values.iter().map(|v| v[i]).product::<i64>(),
                '+' => values.iter().map(|v| v[i]).sum::<i64>(),
                _ => panic!("unknown op {op}"),
            })
            .sum::<i64>()
            .to_string()
    }

    fn solve_part_two(&mut self) -> String {
        let data = self
            .input
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<_>>>();
        let width = data.iter().map(|l| l.len()).max().unwrap();
        let height = data.len();
        let mut sum = 0;
        let mut operands = vec![];
        for column_idx in (0..width).rev() {
            let digits = data[0..(height - 1)]
                .iter()
                .map(|l| l[column_idx])
                .collect::<Vec<_>>();
            if digits.iter().all(|d| *d == ' ') && data[height - 1][column_idx] == ' ' {
                assert!(operands.is_empty());
                continue;
            }
            assert!(!digits.iter().all(|d| *d == ' '));
            let mut operand = 0;
            for d in digits.iter() {
                if d.is_ascii_digit() {
                    operand = operand * 10 + (*d as u8 - b'0') as i64;
                }
            }
            operands.push(operand);
            match data[height - 1][column_idx] {
                '*' => {
                    sum += operands.iter().product::<i64>();
                }
                '+' => {
                    sum += operands.iter().sum::<i64>();
                }
                _ => {
                    continue;
                }
            }
            operands.clear();
        }
        sum.to_string()
    }
}

pub fn solver() -> Day6Solver {
    Day6Solver::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example() {
        let example = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("4277556", s.solve_part_one());
        assert_eq!("3263827", s.solve_part_two());
    }
}
