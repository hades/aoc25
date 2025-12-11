use std::collections::HashMap;

use crate::solver::Solver;

#[derive(Default)]
pub struct Day11Solver {
    devices: HashMap<String, Vec<String>>,
}

#[derive(Clone, Copy, Default)]
struct PathsCount {
    with_nothing: usize,
    with_only_dac: usize,
    with_only_fft: usize,
    with_both: usize,
}

impl Day11Solver {
    fn count_paths<'a>(&'a self, cache: &mut HashMap<&'a str, usize>, from: &'a str) -> usize {
        if from == "out" {
            return 1;
        }
        if let Some(&cached) = cache.get(from) {
            return cached;
        }
        let res = self.devices[&from.to_string()]
            .iter()
            .map(|d| self.count_paths(cache, d))
            .sum::<usize>();
        cache.insert(from, res);
        res
    }

    fn count_paths_two<'a>(
        &'a self,
        cache: &mut HashMap<&'a str, PathsCount>,
        from: &'a str,
    ) -> PathsCount {
        if from == "out" {
            return PathsCount {
                with_nothing: 1,
                ..Default::default()
            };
        }
        if let Some(&cached) = cache.get(from) {
            return cached;
        }
        let mut res = PathsCount::default();
        for p in self.devices[&from.to_string()]
            .iter()
            .map(|d| self.count_paths_two(cache, d))
        {
            if from == "dac" {
                res.with_both += p.with_both + p.with_only_fft;
                res.with_only_dac += p.with_nothing + p.with_only_dac;
            } else if from == "fft" {
                res.with_both += p.with_both + p.with_only_dac;
                res.with_only_fft += p.with_nothing + p.with_only_fft;
            } else {
                res.with_both += p.with_both;
                res.with_nothing += p.with_nothing;
                res.with_only_dac += p.with_only_dac;
                res.with_only_fft += p.with_only_fft;
            }
        }
        cache.insert(from, res);
        res
    }
}

impl Solver for Day11Solver {
    fn presolve(&mut self, input: &str) {
        self.devices = input
            .lines()
            .map(|line| {
                let mut split = line.split(" ");
                let dev = split.next().unwrap().strip_suffix(":").unwrap();
                let conns = split.map(|s| s.to_string()).collect();
                (dev.to_string(), conns)
            })
            .collect();
    }

    fn solve_part_one(&mut self) -> String {
        let mut cache = HashMap::<&str, usize>::new();
        self.count_paths(&mut cache, "you").to_string()
    }

    fn solve_part_two(&mut self) -> String {
        let mut cache = HashMap::<&str, PathsCount>::new();
        self.count_paths_two(&mut cache, "svr")
            .with_both
            .to_string()
    }
}

pub fn solver() -> Day11Solver {
    Default::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn provided_example() {
        let example = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("5", s.solve_part_one());
    }

    #[test]
    fn provided_example_two() {
        let example = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let mut s = solver();
        s.presolve(example);
        assert_eq!("2", s.solve_part_two());
    }
}
