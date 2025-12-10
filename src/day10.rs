use std::fmt::Debug;

use z3::{Optimize, ast::Int};

use crate::solver::Solver;

#[derive(Debug, Default)]
struct Machine {
    light_pattern: usize,
    light_count: u8,
    buttons: Vec<usize>,
    joltage: Vec<usize>,
}

#[derive(Default)]
pub struct Day8Solver {
    machines: Vec<Machine>,
}

impl Day8Solver {}

fn parse_light_pattern(input: &str) -> (usize, u8) {
    let chars = input.chars().collect::<Vec<_>>();
    assert_eq!('[', chars[0]);
    assert_eq!(&']', chars.last().unwrap());
    let mut res = 0usize;
    let mut count = 0u8;
    for i in (1..(chars.len() - 1)).rev() {
        res <<= 1;
        if chars[i] == '#' {
            res |= 1;
        }
        count += 1;
    }
    (res, count)
}

fn parse_button(input: &str) -> usize {
    let input = input.strip_prefix('(').unwrap();
    let input = input.strip_suffix(')').unwrap();
    let mut res = 0usize;
    for n in input.split(",") {
        let i = n.parse::<u8>().unwrap();
        res |= 1 << i;
    }
    res
}

fn parse_joltage(input: &str) -> Vec<usize> {
    let input = input.strip_prefix('{').unwrap();
    let input = input.strip_suffix('}').unwrap();
    input.split(",").map(|v| v.parse().unwrap()).collect()
}

fn solve_joltage_z3(joltage: &[usize], buttons: &[usize]) -> i64 {
    let solver = Optimize::new();
    let button_vars = (0..buttons.len())
        .map(|_| Int::fresh_const("button"))
        .collect::<Vec<_>>();
    let mut joltage_vars = (0..joltage.len())
        .map(|_| Int::from_i64(0))
        .collect::<Vec<_>>();
    for (i, btn) in buttons.iter().enumerate() {
        for j in 0..joltage.len() {
            if *btn & (1usize << j) != 0 {
                *joltage_vars.get_mut(j).unwrap() += &button_vars[i];
            }
        }
        solver.assert(&button_vars[i].ge(0));
    }
    for j in 0..joltage.len() {
        solver.assert(&joltage_vars[j].eq(Int::from_i64(joltage[j] as i64)));
    }
    let sum = button_vars.into_iter().reduce(|a, b| (a + b)).unwrap();
    solver.minimize(&sum);
    match solver.check(&[]) {
        z3::SatResult::Sat => solver
            .get_model()
            .unwrap()
            .eval(&sum, true)
            .unwrap()
            .as_i64()
            .unwrap(),
        _ => panic!(),
    }
}

impl Solver for Day8Solver {
    fn presolve(&mut self, input: &str) {
        for line in input.lines() {
            let mut machine = Machine::default();
            let mut parts = line.split(" ");
            (machine.light_pattern, machine.light_count) =
                parse_light_pattern(parts.next().unwrap());
            for b in parts {
                if b.starts_with('(') {
                    machine.buttons.push(parse_button(b));
                } else {
                    machine.joltage = parse_joltage(b);
                }
            }

            self.machines.push(machine);
        }
    }

    fn solve_part_one(&mut self) -> String {
        self.machines
            .iter()
            .map(|m| {
                let limit = 1usize << m.buttons.len();
                (0..limit)
                    .filter_map(|btn_config| {
                        let mut light_result = 0usize;
                        let mut btn_mask = 1usize;
                        for i in 0..m.buttons.len() {
                            if btn_config & btn_mask != 0 {
                                light_result ^= m.buttons[i];
                            }
                            btn_mask <<= 1;
                        }
                        if light_result == m.light_pattern {
                            Some(btn_config.count_ones())
                        } else {
                            None
                        }
                    })
                    .min()
                    .unwrap()
            })
            .sum::<u32>()
            .to_string()
    }

    fn solve_part_two(&mut self) -> String {
        self.machines
            .iter()
            .map(|m| solve_joltage_z3(&m.joltage, &m.buttons))
            .sum::<i64>()
            .to_string()
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
        let example = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("7", s.solve_part_one());
        assert_eq!("33", s.solve_part_two());
    }
}
