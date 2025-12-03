use aocshared::aoc::aoc::*;
use itertools::Itertools;

const YEAR: i32 = 2025;
const DAY: u32 = 03;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn scan_bank(bank: &Vec<u32>, start: usize, end: usize) -> (i32, u32) {
    (start..end).fold((0 as i32, 0), |acc, cidx| {
        if bank[cidx] > acc.1 {
            (cidx as i32, bank[cidx])
        } else {
            acc
        }
    })
}

fn calc_total_joltage(data: &String, digit_count: usize) -> u64 {
    get_lines_as_grid_u32(data)
        .iter()
        .map(|bank| {
            let mut last: (i32, u32) = (-1, 0);
            (0..digit_count)
                .map(|scan_idx| {
                    last = scan_bank(
                        &bank,
                        (last.0 + 1) as usize,
                        bank.len() - (digit_count - 1 - scan_idx),
                    );
                    char::from_digit(last.1, 10).unwrap()
                })
                .collect_vec()
                .iter()
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .sum()
}

fn part1(data: &String) -> u64 {
    calc_total_joltage(data, 2)
}

fn part2(data: &String) -> u64 {
    calc_total_joltage(data, 12)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2025_03_ep1() {
        assert_eq!(357, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_03_ep2() {
        assert_eq!(3121910778619, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_03_rp1() {
        assert_eq!(17452, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_03_rp2() {
        assert_eq!(173300819005913, part2(&get_input(YEAR, DAY)));
    }
}
