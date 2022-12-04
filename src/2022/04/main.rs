use aocshared::*;
use itertools::Itertools;
use range_ext::intersect::Intersect;
use std::ops::Range;

const YEAR: i32 = 2022;
const DAY: u32 = 04;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn to_range(range: &str) -> Range<u64> {
    let r = range
        .split("-")
        .collect_vec()
        .iter()
        .map(|r| r.parse::<u64>().unwrap())
        .collect_vec();
    Range {
        start: r[0],
        end: r[1] + 1,
    }
}

fn part1(data: &String) -> u64 {
    get_lines_as_strs(data)
        .iter()
        .map(|line| line.split(",").collect_vec())
        .map(|rngs| (to_range(rngs[0]), to_range(rngs[1])))
        .map(|(r1, r2)| {
            let inter = r1.intersect(&r2);
            inter.is_over() || inter.is_within()
        })
        .map(|b| b as u64)
        .sum()
}

fn part2(data: &String) -> u64 {
    get_lines_as_strs(data)
        .iter()
        .map(|line| line.split(",").collect_vec())
        .map(|rngs| (to_range(rngs[0]), to_range(rngs[1])))
        .map(|(r1, r2)| r1.intersect(&r2).is_any())
        .map(|b| b as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2022_04_ep1() {
        assert_eq!(2, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_04_ep2() {
        assert_eq!(4, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_04_rp1() {
        assert_eq!(498, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_04_rp2() {
        assert_eq!(859, part2(&get_input(YEAR, DAY)));
    }
}
