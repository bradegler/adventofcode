use aocshared::aoc::aoc::*;
use regex::Regex;

const YEAR: i32 = 2024;
const DAY: u32 = 03;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i64 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mul = Regex::new(r"mul\((?P<left>\d+),(?P<right>\d+)\)").unwrap();
    re.find_iter(data)
        .into_iter()
        .map(|m| {
            let caps = mul.captures(m.as_str()).unwrap();
            caps["left"].parse::<i64>().unwrap() * caps["right"].parse::<i64>().unwrap()
        })
        .sum()
}

fn part2(data: &String) -> i64 {
    let mul_fn = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let do_fn = Regex::new(r"do\(\)").unwrap();
    let dont_fn = Regex::new(r"don't\(\)").unwrap();
    let mul_re = Regex::new(r"mul\((?P<left>\d+),(?P<right>\d+)\)").unwrap();

    let mut instructions = vec![
        mul_fn.find_iter(data).collect::<Vec<_>>(),
        do_fn.find_iter(data).collect::<Vec<_>>(),
        dont_fn.find_iter(data).collect::<Vec<_>>(),
    ]
    .concat();
    instructions.sort_by(|m1, m2| m1.start().cmp(&m2.start()));

    instructions
        .iter()
        .fold((true, 0), |(enabled, acc), ins| match ins.as_str() {
            is_do if is_do == "do()" => (true, acc),
            is_dont if is_dont == "don't()" => (false, acc),
            mul if enabled => {
                let caps = mul_re.captures(mul).unwrap();
                (
                    enabled,
                    acc + caps["left"].parse::<i64>().unwrap()
                        * caps["right"].parse::<i64>().unwrap(),
                )
            }
            _ => (enabled, acc),
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2024_03_ep1() {
        assert_eq!(161, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_03_ep2() {
        assert_eq!(48, part2(&get_test_input_part(YEAR, DAY, 2)));
    }

    #[test]
    fn t2024_03_rp1() {
        assert_eq!(161289189, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_03_rp2() {
        assert_eq!(83595109, part2(&get_input(YEAR, DAY)));
    }
}
