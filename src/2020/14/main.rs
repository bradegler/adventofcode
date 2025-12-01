use aocshared::aoc::aoc::*;
use hashbrown::HashMap;
use regex::Regex;

const YEAR: i32 = 2020;
const DAY: u32 = 14;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let input = get_lines_as_strs(data);
    let mut and_mask = 0u64;
    let mut or_mask = 0u64;
    let mut mem = HashMap::new();
    let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    for line in input {
        if line.starts_with("mask") {
            let mask = line.split_at(7).1;
            and_mask = usize::from_str_radix(&mask.replace("X", "1"), 2).unwrap() as u64;
            or_mask = usize::from_str_radix(&mask.replace("X", "0"), 2).unwrap() as u64;
        } else {
            let caps = re.captures(line).unwrap();
            let addr = caps[1].parse::<u64>().unwrap();
            let val = caps[2].parse::<u64>().unwrap();
            let masked_val = val & and_mask | or_mask;
            mem.insert(addr, masked_val);
        }
    }
    mem.values().sum()
}

fn part2(data: &String) -> u64 {
    let input = get_lines_as_strs(data);
    let mut and_mask = 0u64;
    let mut or_mask = 0u64;
    let mut mem = HashMap::new();
    let mut mask = "";
    let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    for line in input {
        if line.starts_with("mask") {
            mask = line.split_at(7).1;
            and_mask = usize::from_str_radix(&mask.replace("X", "1"), 2).unwrap() as u64;
            or_mask = usize::from_str_radix(&mask.replace("X", "0"), 2).unwrap() as u64;
        } else {
            let caps = re.captures(line).unwrap();
            let addr = caps[1].parse::<u64>().unwrap();
            let val = caps[2].parse::<u64>().unwrap();
            let masked_addr = addr & and_mask | or_mask;
            // Now fan out for every combination of X bits that "float"
            for _ in 0..mask.len() {}
            mem.insert(masked_addr, val);
        }
    }
    #[allow(unused)]
    let s: u64 = mem.values().sum();
    // @TODO finish
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2020_14_ep1() {
        assert_eq!(165, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_14_ep2() {
        assert_eq!(0, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_14_rp1() {
        assert_eq!(6317049172545, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_14_rp2() {
        assert_eq!(0, part2(&get_input(YEAR, DAY)));
    }
}
