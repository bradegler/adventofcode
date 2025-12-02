use aocshared::aoc::aoc::*;
use itertools::Itertools;

const YEAR: i32 = 2025;
const DAY: u32 = 02;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    data.trim()
        .split(',')
        .map(|s| s.split('-').collect::<Vec<&str>>())
        .map(|v| (v[0].parse::<u64>().unwrap(), v[1].parse::<u64>().unwrap()))
        .collect_vec()
        .iter()
        .fold(0, |mut acc, (a, b)| {
            for i in *a..=*b {
                let i_str = i.to_string();
                let mid = i_str.len() / 2;
                if &i_str[..mid] == &i_str[mid..] {
                    acc += i;
                }
            }
            acc
        })
}

fn part2(data: &String) -> u64 {
    data.trim()
        .split(',')
        .map(|s| s.split('-').collect::<Vec<&str>>())
        .map(|v| (v[0].parse::<u64>().unwrap(), v[1].parse::<u64>().unwrap()))
        .collect_vec()
        .iter()
        .fold(0, |mut acc, (a, b)| {
            for i in *a..=*b {
                let i_str = i.to_string() + &i.to_string();
                let loc = i_str[1..].find(&i.to_string()).unwrap_or(i_str.len());
                if loc < i.to_string().len() - 1 {
                    acc += i;
                }
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2025_02_ep1() {
        assert_eq!(1227775554, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_02_ep2() {
        assert_eq!(4174379265, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_02_rp1() {
        assert_eq!(38310256125, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_02_rp2() {
        assert_eq!(58961152806, part2(&get_input(YEAR, DAY)));
    }
}
