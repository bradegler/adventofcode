use aocshared::*;
use core::cmp::max;
use core::cmp::min;
use regex::Regex;
use std::collections::HashMap;

const YEAR: i32 = 2021;
const DAY: u32 = 22;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cuboid {
    on: bool,
    xr: (i64, i64),
    yr: (i64, i64),
    zr: (i64, i64),
}

impl Cuboid {
    fn new(on: bool, xr: (i64, i64), yr: (i64, i64), zr: (i64, i64)) -> Self {
        Self { on: on, xr, yr, zr }
    }

    fn bounded(&self, low: i64, high: i64) -> bool {
        self.xr.0 >= low
            && self.xr.1 <= high
            && self.yr.0 >= low
            && self.yr.1 <= high
            && self.zr.0 >= low
            && self.zr.1 <= high
    }

    fn intersection(self, mut target: Cuboid) -> Option<Cuboid> {
        let mut this = self.clone();
        let (l, r) = (max(this.xr.0, target.xr.0), min(this.xr.1, target.xr.1));
        if l > r {
            return None;
        }
        this.xr = (l, r);
        target.xr = (l, r);
        let (l, r) = (max(this.yr.0, target.yr.0), min(this.yr.1, target.yr.1));
        if l > r {
            return None;
        }
        this.yr = (l, r);
        target.yr = (l, r);
        let (l, r) = (max(this.zr.0, target.zr.0), min(this.zr.1, target.zr.1));
        if l > r {
            return None;
        }
        this.zr = (l, r);
        target.zr = (l, r);

        Some(this)
    }
}

fn parse_input(data: &String) -> Vec<Cuboid> {
    get_lines_as_strs(&data)
        .iter()
        .map(|l| {
            let re =
                Regex::new(r"^(on|off) x=(-?\d+\.\.-?\d+),y=(-?\d+\.\.-?\d+),z=(-?\d+\.\.-?\d+)$")
                    .unwrap();
            let caps = re.captures(l).unwrap();
            let ins = caps.get(1).unwrap().as_str();
            let x = caps
                .get(2)
                .unwrap()
                .as_str()
                .split_once("..")
                .map(|(l, r)| (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap()))
                .unwrap();
            let y = caps
                .get(3)
                .unwrap()
                .as_str()
                .split_once("..")
                .map(|(l, r)| (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap()))
                .unwrap();
            let z = caps
                .get(4)
                .unwrap()
                .as_str()
                .split_once("..")
                .map(|(l, r)| (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap()))
                .unwrap();
            Cuboid::new(ins == "on", x, y, z)
        })
        .collect()
}

fn part1(data: &String) -> i64 {
    let cuboids = parse_input(&data);
    let mut weights: HashMap<Cuboid, i64> = HashMap::new();
    for op_cube in cuboids {
        if !op_cube.bounded(-50, 50) {
            continue;
        }
        let mut new_weights = weights.clone();
        for (cube, weight) in weights {
            if let Some(intersection) = cube.intersection(op_cube) {
                *new_weights.entry(intersection).or_insert(0) -= weight;
            }
        }
        if op_cube.on {
            *new_weights.entry(op_cube).or_insert(0) += 1;
        }
        weights = new_weights;
    }
    weights
        .into_iter()
        .map(|(c, s)| {
            [c.xr, c.yr, c.zr]
                .into_iter()
                .map(|d| d.1 - d.0 + 1)
                .product::<i64>()
                * s
        })
        .sum::<i64>()
}

fn part2(data: &String) -> i64 {
    let cuboids = parse_input(&data);
    let mut weights: HashMap<Cuboid, i64> = HashMap::new();
    for op_cube in cuboids {
        let mut new_weights = weights.clone();
        for (cube, weight) in weights {
            if let Some(intersection) = cube.intersection(op_cube) {
                *new_weights.entry(intersection).or_insert(0) -= weight;
            }
        }
        if op_cube.on {
            *new_weights.entry(op_cube).or_insert(0) += 1;
        }
        weights = new_weights;
    }
    weights
        .into_iter()
        .map(|(c, s)| {
            [c.xr, c.yr, c.zr]
                .into_iter()
                .map(|d| d.1 - d.0 + 1)
                .product::<i64>()
                * s
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_22_ep1() {
        assert_eq!(474140, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_22_ep2() {
        assert_eq!(2758514936282235, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_22_rp1() {
        assert_eq!(568000, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_22_rp2() {
        assert_eq!(1177411289280259, part2(&get_input(YEAR, DAY)));
    }
}
