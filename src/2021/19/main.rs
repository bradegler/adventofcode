use aocshared::aoc::aoc::*;
use std::collections::HashSet;

use itertools::Itertools;

const YEAR: i32 = 2021;
const DAY: u32 = 19;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

struct Scanner {
    dist: (i32, i32, i32),
    beacons: Vec<(i32, i32, i32)>,
}

fn part1(data: &String) -> usize {
    let mut scanners = data
        .split("\n\n")
        .enumerate()
        .map(|(_, s)| Scanner {
            dist: (0, 0, 0),
            beacons: s
                .lines()
                .skip(1)
                .map(|l| {
                    let (x, tmp) = l.split_once(",").unwrap();
                    let (y, z) = tmp.split_once(",").unwrap();
                    (x, y, z)
                })
                .map(|(x, y, z)| {
                    (
                        x.parse::<i32>().unwrap(),
                        y.parse::<i32>().unwrap(),
                        z.parse::<i32>().unwrap(),
                    )
                })
                .collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();
    println!("{:?}", scanners.len());
    // Scanner 0 is the point of reference, capture what it sees as the starting point
    let mut target_scans = scanners
        .remove(0)
        .beacons
        .into_iter()
        .collect::<HashSet<_>>();
    // Iterate over the remaining scanners
    let mut processed = vec![];
    while !scanners.is_empty() {
        for i in (0..scanners.len()).rev() {
            if merge(&mut target_scans, &mut scanners[i]) {
                processed.push(scanners.swap_remove(i));
            }
        }
    }
    target_scans.len()
}

fn part2(data: &String) -> i32 {
    let mut scanners = data
        .split("\n\n")
        .enumerate()
        .map(|(_, s)| Scanner {
            dist: (0, 0, 0),
            beacons: s
                .lines()
                .skip(1)
                .map(|l| {
                    let (x, tmp) = l.split_once(",").unwrap();
                    let (y, z) = tmp.split_once(",").unwrap();
                    (x, y, z)
                })
                .map(|(x, y, z)| {
                    (
                        x.parse::<i32>().unwrap(),
                        y.parse::<i32>().unwrap(),
                        z.parse::<i32>().unwrap(),
                    )
                })
                .collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();
    println!("{:?}", scanners.len());
    // Scanner 0 is the point of reference, capture what it sees as the starting point
    let mut target_scans = scanners
        .remove(0)
        .beacons
        .into_iter()
        .collect::<HashSet<_>>();
    // Iterate over the remaining scanners
    let mut processed = vec![];
    while !scanners.is_empty() {
        for i in (0..scanners.len()).rev() {
            if merge(&mut target_scans, &mut scanners[i]) {
                processed.push(scanners.swap_remove(i));
            }
        }
    }
    processed
        .iter()
        .map(|s| s.dist)
        .permutations(2)
        .map(|p| {
            let (x1, y1, z1) = p[0];
            let (x2, y2, z2) = p[1];
            (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
        })
        .max()
        .unwrap()
}

fn rotate((x, y, z): (i32, i32, i32), rot: usize) -> (i32, i32, i32) {
    match rot {
        0 => (x, y, z),
        1 => (x, z, -y),
        2 => (x, -y, -z),
        3 => (x, -z, y),
        4 => (y, x, -z),
        5 => (y, z, x),
        6 => (y, -x, z),
        7 => (y, -z, -x),
        8 => (z, x, y),
        9 => (z, y, -x),
        10 => (z, -x, -y),
        11 => (z, -y, x),
        12 => (-x, y, -z),
        13 => (-x, z, y),
        14 => (-x, -y, z),
        15 => (-x, -z, -y),
        16 => (-y, x, z),
        17 => (-y, z, -x),
        18 => (-y, -x, -z),
        19 => (-y, -z, x),
        20 => (-z, x, -y),
        21 => (-z, y, x),
        22 => (-z, -x, y),
        23 => (-z, -y, -x),
        _ => unreachable!(),
    }
}

fn merge(total_scan: &mut HashSet<(i32, i32, i32)>, scanner: &mut Scanner) -> bool {
    for rot in 0..24 {
        let rotated = scanner
            .beacons
            .iter()
            .map(|&v| rotate(v, rot))
            .collect::<Vec<_>>();
        let distances = total_scan
            .iter()
            .cartesian_product(&rotated)
            .map(|((x1, y1, z1), (x2, y2, z2))| (x1 - x2, y1 - y2, z1 - z2));
        for (dx, dy, dz) in distances {
            let translated = rotated.iter().map(|(x, y, z)| (x + dx, y + dy, z + dz));
            if translated
                .clone()
                .filter(|v| total_scan.contains(v))
                .count()
                >= 12
            {
                total_scan.extend(translated);
                scanner.dist = (dx, dy, dz);
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2021_19_ep1() {
        assert_eq!(79, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_19_ep2() {
        assert_eq!(3621, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_19_rp1() {
        assert_eq!(428, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_19_rp2() {
        assert_eq!(12140, part2(&get_input(YEAR, DAY)));
    }
}
