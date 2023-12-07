use aocshared::aoc::aoc::*;
use itertools::Itertools;

const YEAR: i32 = 2023;
const DAY: u32 = 06;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let lines = get_lines_as_strs_rm_empty(data);
    let times = lines[0]
        .split_whitespace()
        .map(|s| s.parse::<u64>())
        .flatten()
        .collect::<Vec<u64>>();
    let distances = lines[1]
        .split_whitespace()
        .map(|s| s.parse::<u64>())
        .flatten()
        .collect::<Vec<u64>>();
    (0..times.len())
        .map(|i| Race::new(times[i], distances[i]))
        .map(|r| r.win_conditions())
        .fold(1, |acc, cnt| acc * cnt.len() as u64)
}

fn part2(data: &String) -> u64 {
    let values = get_lines_as_strs_rm_empty(data)
        .iter()
        .map(|s| s.replace(" ", ""))
        .map(|s| s.split_once(":").unwrap().1.parse::<u64>().unwrap())
        .collect_vec();
    Race::new(*values.get(0).unwrap(), *values.get(1).unwrap())
        .win_conditions()
        .len() as u64
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn win_conditions(&self) -> Vec<u64> {
        (0..self.time)
            .map(|d| {
                if ((self.time - d) * d) > self.distance {
                    Some(d)
                } else {
                    None
                }
            })
            .flatten()
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2023_06_ep1() {
        assert_eq!(288, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_06_ep2() {
        assert_eq!(71503, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_06_rp1() {
        assert_eq!(1731600, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_06_rp2() {
        assert_eq!(40087680, part2(&get_input(YEAR, DAY)));
    }
}
