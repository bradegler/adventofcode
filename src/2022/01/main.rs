use aocshared::*;
use itertools::Itertools;

const YEAR: i32 = 2022;
const DAY: u32 = 01;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let lines  =  get_lines_as_strs(data);
    let elves = lines.split(|l| l.trim().len() == 0);
    elves.into_iter().map(|elf| elf.iter().map(|s| s.parse::<u64>().unwrap()).sum::<u64>()
    ).sorted().rev().take(1).sum()
}

fn part2(data: &String) -> u64 {
    let lines  =  get_lines_as_strs(data);
    let elves = lines.split(|l| l.trim().len() == 0);
    elves.into_iter().map(|elf| elf.iter().map(|s| s.parse::<u64>().unwrap()).sum::<u64>()
    ).sorted().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2022_01_ep1() {
        assert_eq!(24000, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_01_ep2() {
        assert_eq!(45000, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_01_rp1() {
        assert_eq!(71780, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_01_rp2() {
        assert_eq!(212489, part2(&get_input(YEAR, DAY)));
    }
}
