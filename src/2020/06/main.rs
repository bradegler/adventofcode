use aocshared::get_input;
use std::collections::HashSet;

const YEAR: i32 = 2020;
const DAY: u32 = 06;

fn main() {
    let i = get_input(YEAR, DAY);
    part1(&i);
    part2(&i);
}

fn part1(data: &String) -> i32 {
    println!("Part 1");
    let mut input = data.clone();
    input.push_str("\n");
    let result = input
        .replace("\n\n", "|")
        .replace("\n", "")
        .split("|")
        .map(|x| x.chars().collect::<HashSet<char>>().len())
        .sum::<usize>();
    println!("Part 1 Result: {}", result);
    return result as i32;
}

fn part2(data: &String) -> i32 {
    println!("Part 2");
    let mut input = data.clone();
    input.push_str("\n");
    let result = input
        .replace("\n\n", "|")
        .replace("\n", " ")
        .split("|")
        .map(|x| {
            x.replace("|", "")
                .split(" ")
                .map(|y| y.chars().collect::<HashSet<char>>())
                .reduce(|acc, x| acc.intersection(&x).cloned().collect::<HashSet<char>>())
                .map(|x| x.len())
                .unwrap_or(0)
        })
        .sum::<usize>();
    println!("Part 2 Result: {}", result);
    return result as i32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2020_06_ep1() {
        assert_eq!(11, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_06_ep2() {
        assert_eq!(6, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_06_rp1() {
        assert_eq!(6630, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_06_rp2() {
        assert_eq!(3437, part2(&get_input(YEAR, DAY)));
    }
}
