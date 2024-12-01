use std::collections::BinaryHeap;

use aocshared::aoc::aoc::*;
use regex::Regex;

const YEAR: i32 = 2024;
const DAY: u32 = 01;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn read_and_sort_data(data: &String) -> (Vec<i64>, Vec<i64>) {
    let re = Regex::new(r"(?P<left>\d+) +(?P<right>\d+)").unwrap();
    let (left, right) = get_lines_as_strs_rm_empty(data).iter().fold(
        (BinaryHeap::<i64>::new(), BinaryHeap::<i64>::new()),
        |(mut left, mut right), line| {
            let caps = re.captures(line).unwrap();
            left.push(caps["left"].parse::<i64>().unwrap());
            right.push(caps["right"].parse::<i64>().unwrap());
            (left, right)
        },
    );
    (left.into_sorted_vec(), right.into_sorted_vec())
}

fn part1(data: &String) -> u64 {
    let (left, right) = read_and_sort_data(data);
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (r - l).abs())
        .sum::<i64>()
        .try_into()
        .unwrap()
}

fn part2(data: &String) -> u64 {
    let (left, right) = read_and_sort_data(data);
    left.iter()
        .map(|l| right.iter().filter(|r| **r == *l).count() as i64 * l)
        .sum::<i64>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2024_01_ep1() {
        assert_eq!(11, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_01_ep2() {
        assert_eq!(31, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_01_rp1() {
        assert_eq!(1319616, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_01_rp2() {
        assert_eq!(27267728, part2(&get_input(YEAR, DAY)));
    }
}
