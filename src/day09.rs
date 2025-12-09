use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet},
};

use itertools::Itertools;

use crate::solver::Solver;

#[derive(Default)]
pub struct Day8Solver {
    tiles: Vec<(i64, i64)>,
    rectangles: Vec<((i64, i64), (i64, i64))>,
}

impl Day8Solver {}

#[derive(Debug)]
enum ScanState {
    //     +--+
    // x   |  |
    //     +--+
    Outside,
    //     + --+
    //     |   |
    //     +x--+
    OnEdge(i64, bool),
    //     +--+
    //     |x |
    //     +--+
    Inside(i64),
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
        self.rectangles = self
            .tiles
            .iter()
            .cartesian_product(self.tiles.iter())
            .map(|(a, b)| ((a.0.min(b.0), a.1.min(b.1)), (a.0.max(b.0), a.1.max(b.1))))
            .unique()
            .collect();
    }

    fn solve_part_one(&mut self) -> String {
        self.rectangles
            .iter()
            .map(|(a, b)| ((b.0 - a.0 + 1) * (b.1 - a.1 + 1)).abs())
            .max()
            .unwrap()
            .to_string()
    }

    fn solve_part_two(&mut self) -> String {
        let lines_with_corners_y = self.tiles.iter().map(|t| t.1).collect::<BTreeSet<_>>();
        let mut shape = BTreeMap::<(i64, i64), Vec<(i64, i64)>>::new();
        let mut next_line_to_merge = i64::MIN;
        for &l in lines_with_corners_y.iter() {
            if next_line_to_merge < l - 1 {
                shape.insert((next_line_to_merge, l - 1), vec![]);
            }
            shape.insert((l - 1, l), vec![]);
            shape.insert((l, l + 1), vec![]);
            next_line_to_merge = l + 1;
        }
        shape.insert((next_line_to_merge, next_line_to_merge + 1), vec![]);
        let segments = (0..self.tiles.len())
            .map(|segment_idx| {
                (
                    self.tiles[segment_idx],
                    self.tiles[(segment_idx + 1) % self.tiles.len()],
                )
            })
            .sorted_by_key(|(a, b)| (a.0.min(b.0), (b.0 - a.0).abs()))
            .collect::<Vec<_>>();
        for (&(from_y, to_y), scan_line) in shape.iter_mut() {
            let mut scan_state = ScanState::Outside;
            for (a, b) in segments.iter().cloned() {
                if b.1 == a.1 {
                    if b.1 != from_y {
                        continue;
                    }
                    assert!(matches!(&scan_state, &ScanState::OnEdge(_, _)));
                    scan_line.push((a.0.min(b.0), a.0.max(b.0) + 1));
                } else if b.1.max(a.1) < from_y || b.1.min(a.1) >= to_y {
                    continue;
                } else if b.1 == from_y || a.1 == from_y {
                    assert_eq!(b.0, a.0);
                    let new_wall_direction = (b.1 - a.1).signum();
                    match scan_state {
                        ScanState::Inside(first_inside_x) => {
                            scan_line.push((first_inside_x, a.0));
                            scan_state = ScanState::OnEdge(new_wall_direction, true);
                        }
                        ScanState::OnEdge(wall_direction, was_inside) => {
                            let inside = if wall_direction == new_wall_direction {
                                !was_inside
                            } else {
                                was_inside
                            };
                            scan_state = if inside {
                                ScanState::Inside(b.0 + 1)
                            } else {
                                ScanState::Outside
                            };
                        }
                        ScanState::Outside => {
                            scan_state = ScanState::OnEdge(new_wall_direction, false);
                        }
                    }
                } else if from_y < b.1.max(a.1) && from_y > b.1.min(a.1) {
                    assert_eq!(b.0, a.0);
                    match scan_state {
                        ScanState::Inside(first_inside_x) => {
                            scan_line.push((first_inside_x, a.0 + 1));
                            scan_state = ScanState::Outside;
                        }
                        ScanState::Outside => {
                            scan_state = ScanState::Inside(a.0);
                        }
                        _ => unreachable!(),
                    }
                } else {
                    unreachable!()
                }
            }
        }
        self.rectangles
            .iter()
            .map(|(a, b)| (((b.0 - a.0 + 1) * (b.1 - a.1 + 1)).abs(), a, b))
            .sorted_by_key(|(area, _, _)| Reverse(*area))
            .find(|&(_, begin, end)| {
                let mut rect_begin_y = begin.1;
                for (&(from_y, to_y), scan_line) in shape.iter() {
                    if rect_begin_y > end.1 {
                        break;
                    }
                    if to_y <= rect_begin_y {
                        continue;
                    }
                    if from_y > rect_begin_y {
                        break;
                    }
                    let mut rect_begin_x = begin.0;
                    for &(scan_start_x, scan_end_x) in scan_line {
                        if rect_begin_x > end.0 {
                            break;
                        }
                        if scan_end_x <= rect_begin_x {
                            continue;
                        }
                        if scan_start_x > rect_begin_x {
                            break;
                        }
                        rect_begin_x = scan_end_x;
                    }
                    if rect_begin_x <= end.0 {
                        return false;
                    }
                    rect_begin_y = to_y;
                }
                rect_begin_y > end.1
            })
            .unwrap()
            .0
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
        assert_eq!("24", s.solve_part_two());
    }
}
