use aocshared::*;
use num_integer::lcm;

const YEAR: i32 = 2020;
const DAY: u32 = 13;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i32 {
    let input = get_lines_as_strs(data);
    let time = input[0].parse::<i32>().unwrap();
    let busses = input[1]
        .split(",")
        .filter(|s| s != &"x")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("{:?}", time);
    println!("{:?}", busses);

    busses
        .iter()
        .map(|b| (b, b - time % b))
        .min_by_key(|(_, d)| *d)
        .map(|(b, m)| b * m)
        .unwrap()
}

fn part2(data: &String) -> i64 {
    let input = get_lines_as_strs(data);
    let time = input[0].parse::<i64>().unwrap();
    let busses = input[1]
        .split(",")
        .enumerate()
        .filter(|(_, s)| s != &"x")
        .map(|(i, s)| (i as i64, s.parse::<i64>().unwrap()))
        .collect::<Vec<(i64, i64)>>();

    println!("{:?}", time);
    println!("{:?}", busses);
    let mut solution = busses[0].1;
    let mut step = busses[0].1;

    for bus in busses.iter().skip(1).copied() {
        while (solution + bus.0) % bus.1 != 0 {
            solution += step;
        }
        step = lcm(step, bus.1);
    }
    solution
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2020_13_ep1() {
        assert_eq!(295, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_13_ep2() {
        assert_eq!(1068781, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_13_rp1() {
        assert_eq!(115, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_13_rp2() {
        assert_eq!(756261495958122, part2(&get_input(YEAR, DAY)));
    }
}
