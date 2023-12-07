use aocshared::aoc::aoc::*;

const YEAR: i32 = 2021;
const DAY: u32 = 01;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i32 {
    get_lines_as_numbers(data)[..]
        .windows(2)
        .fold(0, |acc, tpl| acc + if tpl[1] > tpl[0] { 1 } else { 0 })
}

fn part2(data: &String) -> i32 {
    get_lines_as_numbers(data)[..]
        .windows(3)
        .map(|tpl| tpl.into_iter().fold(0, |a, b| a + b))
        .collect::<Vec<u32>>()[..]
        .windows(2)
        .fold(0, |acc, tpl| acc + if tpl[1] > tpl[0] { 1 } else { 0 })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2021_01_ep1() {
        assert_eq!(7, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_01_ep2() {
        assert_eq!(5, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_01_rp1() {
        assert_eq!(1228, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_01_rp2() {
        assert_eq!(1257, part2(&get_input(YEAR, DAY)));
    }
}
