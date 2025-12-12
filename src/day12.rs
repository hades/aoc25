use array2d::Array2D;
use itertools::Itertools;

use crate::solver::Solver;

#[derive(Debug, Default)]
struct Shape {
    rotations: Vec<Array2D<bool>>,
}

#[derive(Default)]
pub struct Day12Solver {
    shapes: Vec<Shape>,
    problems: Vec<(usize, usize, Vec<usize>)>,
}

fn shape_set_iter(shape: &Array2D<bool>) -> impl Iterator<Item = (usize, usize)> {
    (0..shape.num_rows())
        .cartesian_product(0..shape.num_columns())
        .filter(|(x, y)| shape[(*x, *y)])
}

impl Day12Solver {
    fn solve_one_recursive(&self, board: &mut Array2D<bool>, cardinalities: &[usize]) -> bool {
        if cardinalities.iter().all(|x| *x == 0) {
            return true;
        }
        let mut new_cardinalities = cardinalities.to_vec();
        if let Some((shape_idx, card)) = new_cardinalities
            .iter_mut()
            .enumerate()
            .find(|(_, x)| **x != 0)
        {
            *card -= 1;
            for rotation in self.shapes[shape_idx].rotations.iter() {
                for px in 0..(board.num_rows() - rotation.num_rows() + 1) {
                    for py in 0..(board.num_columns() - rotation.num_columns() + 1) {
                        if !shape_set_iter(rotation).all(|(x, y)| !board[(x + px, y + py)]) {
                            continue;
                        }
                        shape_set_iter(rotation).for_each(|(x, y)| board[(x + px, y + py)] = true);
                        if self.solve_one_recursive(board, &new_cardinalities) {
                            return true;
                        }
                        shape_set_iter(rotation).for_each(|(x, y)| board[(x + px, y + py)] = false);
                    }
                }
            }
        }
        false
    }
}

fn rotate(array: &Array2D<bool>) -> Array2D<bool> {
    assert_eq!(3, array.num_columns());
    assert_eq!(3, array.num_rows());
    let mut result = array.clone();
    result[(0, 2)] = array[(0, 0)];
    result[(1, 2)] = array[(0, 1)];
    result[(2, 2)] = array[(0, 2)];
    result[(0, 1)] = array[(1, 0)];
    result[(1, 1)] = array[(1, 1)];
    result[(2, 1)] = array[(1, 2)];
    result[(0, 0)] = array[(2, 0)];
    result[(1, 0)] = array[(2, 1)];
    result[(2, 0)] = array[(2, 2)];
    result
}

impl Solver for Day12Solver {
    fn presolve(&mut self, input: &str) {
        let mut lines = input.lines().peekable();
        loop {
            if lines.peek().unwrap().contains("x") {
                break;
            }
            self.shapes.push(Default::default());
            lines.next().unwrap();
            let mut rows: Vec<Vec<bool>> = vec![];
            while let Some(line) = lines.next()
                && !line.is_empty()
            {
                rows.push(line.chars().map(|ch| ch == '#').collect());
            }
            let rotation1 = Array2D::from_rows(&rows).unwrap();
            let rotation2 = rotate(&rotation1);
            if rotation1 == rotation2 {
                self.shapes.last_mut().unwrap().rotations = vec![rotation1];
                continue;
            }
            let rotation3 = rotate(&rotation2);
            if rotation3 == rotation1 {
                self.shapes.last_mut().unwrap().rotations = vec![rotation1, rotation2];
                continue;
            }
            let rotation4 = rotate(&rotation3);
            self.shapes.last_mut().unwrap().rotations =
                vec![rotation1, rotation2, rotation3, rotation4];
        }
        self.problems = lines
            .map(|line| {
                let (size, shapes) = line.split_once(": ").unwrap();
                let (w, h) = size.split_once("x").unwrap();
                (
                    w.parse().unwrap(),
                    h.parse().unwrap(),
                    shapes.split(" ").map(|s| s.parse().unwrap()).collect(),
                )
            })
            .collect();
    }

    fn solve_part_one(&mut self) -> String {
        let mut counter = 0;
        for (w, h, cardinalities) in self.problems.iter() {
            if cardinalities
                .iter()
                .zip(self.shapes.iter())
                .map(|(card, shape)| {
                    *card
                        * shape.rotations[0]
                            .elements_row_major_iter()
                            .map(|x| if *x { 1 } else { 0 })
                            .sum::<usize>()
                })
                .sum::<usize>()
                > (*w * *h)
            {
                continue;
            }
            let mut board = Array2D::filled_with(false, *w, *h);
            if self.solve_one_recursive(&mut board, cardinalities) {
                counter += 1;
            }
        }
        counter.to_string()
    }

    fn solve_part_two(&mut self) -> String {
        "".into()
    }
}

pub fn solver() -> Day12Solver {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example() {
        let example = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("2", s.solve_part_one());
    }
}
