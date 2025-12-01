/// A solver for both parts of one day's Advent of Code puzzles.
pub trait Solver {
    /// Preliminary computations for both parts of the puzzle, input parsing,
    /// building necessary data structures, etc.
    fn presolve(&mut self, input: &str);

    /// Solve and return the solution for the first part of the puzzle.
    fn solve_part_one(&mut self) -> String;

    /// Solve and return the solution for the second part of the puzzle.
    fn solve_part_two(&mut self) -> String;
}
