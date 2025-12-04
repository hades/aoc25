use std::collections::HashSet;

use crate::solver::Solver;

#[derive(Default)]
pub struct Day4Solver {
    input: HashSet<(usize, usize)>,
}

fn removable_rolls(input: &HashSet<(usize, usize)>) -> impl Iterator<Item = (usize, usize)> {
    input
        .iter()
        .filter(|&(i, j)| {
            let mut neighbours = 0;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ni = i.wrapping_add_signed(di);
                    let nj = j.wrapping_add_signed(dj);
                    if input.contains(&(ni, nj)) {
                        neighbours += 1;
                    }
                }
            }
            neighbours < 4
        })
        .cloned()
}

impl Solver for Day4Solver {
    fn presolve(&mut self, input: &str) {
        self.input = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(j, ch)| match ch {
                        '@' => Some((i, j)),
                        _ => None,
                    })
            })
            .collect();
    }

    fn solve_part_one(&mut self) -> String {
        removable_rolls(&self.input).count().to_string()
    }

    fn solve_part_two(&mut self) -> String {
        let mut rolls = self.input.clone();
        loop {
            let remove_me = removable_rolls(&rolls).collect::<Vec<_>>();
            remove_me.iter().for_each(|r| {
                rolls.remove(r);
            });
            if remove_me.is_empty() {
                break;
            }
        }
        (self.input.len() - rolls.len()).to_string()
    }
}

pub fn solver() -> Day4Solver {
    Day4Solver::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example() {
        let example = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("13", s.solve_part_one());
        assert_eq!("43", s.solve_part_two());
    }
}
