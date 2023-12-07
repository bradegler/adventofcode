use aocshared::aoc::aoc::*;
use itertools::Itertools;
use range_ext::intersect::Intersect;

const YEAR: i32 = 2022;
const DAY: u32 = 04;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    get_lines_as_strs(data)
        .iter()
        .map(|line| line.split(",").map(|r| to_range(r, true)).collect_vec())
        .map(|rngs| {
            let inter = rngs[0].intersect(&rngs[1]);
            (inter.is_over() || inter.is_within()) as u64
        })
        .sum()
}

fn part2(data: &String) -> u64 {
    get_lines_as_strs(data)
        .iter()
        .map(|line| line.split(",").map(|r| to_range(r, true)).collect_vec())
        .map(|rngs| rngs[0].intersect(&rngs[1]).is_any() as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
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
