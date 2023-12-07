use aocshared::aoc::aoc::*;
const YEAR: i32 = 2019;
const DAY: u32 = 02;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(_data: &String) -> u64 {
    0
}

fn part2(_data: &String) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2019_02_ep1() {
        assert_eq!(0, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2019_02_ep2() {
        assert_eq!(0, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2019_02_rp1() {
        assert_eq!(0, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2019_02_rp2() {
        assert_eq!(0, part2(&get_input(YEAR, DAY)));
    }
}
