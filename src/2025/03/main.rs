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

fn scan_bank(bank: &Vec<u32>, start: usize, end: usize) -> (usize, u32) {
    let mut highest_idx = 0;
    let mut highest_val = 0;
    for cidx in start..end {
        let val = bank[cidx];
        if val > highest_val {
            highest_idx = cidx;
            highest_val = val;
        }
    }
    (highest_idx, highest_val)
}

fn part1(data: &String) -> u64 {
    let banks = get_lines_as_grid_u32(data);
    let mut total_joltage = 0;
    for ridx in 0..banks.len() {
        let bank = &banks[ridx];
        let first = scan_bank(&bank, 0, bank.len() - 1);
        let second = scan_bank(&bank, first.0 + 1, bank.len());
        let bank_joltage = first.1 * 10 + second.1;
        println!(
            "Bank {}: {:?} {:?}-{:?} {}",
            ridx, bank, first, second, bank_joltage
        );
        total_joltage += bank_joltage;
    }
    total_joltage as u64
}

fn part2(data: &String) -> u64 {
    let banks = get_lines_as_grid_u32(data);
    let mut total_joltage: u64 = 0;
    let joltages = banks
        .iter()
        .enumerate()
        .map(|(ridx, bank)| {
            let bank_len = bank.len();
            let mut last = (0, 0);
            let mut bank_joltage = "".to_owned();
            for scan_idx in 0..12 {
                let start = if scan_idx == 0 { 0 } else { last.0 + 1 };
                let max = scan_bank(&bank, start, bank_len - (11 - scan_idx));
                last = max;
                bank_joltage.push_str(&max.1.to_string());
            }
            println!("Bank {}: {:?} = {}", ridx, bank, bank_joltage);
            bank_joltage.parse::<u64>().unwrap()
        })
        .collect_vec();
    total_joltage = joltages.iter().sum();
    println!("Joltages: {:?} == {:?}", joltages, total_joltage);
    total_joltage
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
