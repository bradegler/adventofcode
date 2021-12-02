use aocf::Aoc;

const YEAR: i32 = 2021;
const DAY: u32 = 1;

fn get_input(year: i32, day: u32) -> String {
    println!("Advent of Code {} - Day {}", year, day);

    let mut aoc = Aoc::new().year(Some(year)).day(Some(day)).init().unwrap();

    // Get the brief
    let brief = aoc.get_brief(false);
    if let Ok(brief) = brief {
        println!("{}", brief);
    }

    // Get input data (don't force)
    if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        panic!("No input found");
    }
}

fn main() {
    let i = get_input(YEAR, DAY);
    part1(&i);
    part2(&i);
}

fn part1(data: &String) -> i32 {
    println!("Part 1");
    let count = data
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()[..]
        .windows(2)
        .fold(0, |acc, tpl| acc + if tpl[1] > tpl[0] { 1 } else { 0 });
    println!("Part 1 Result: {}", count);
    return count;
}

fn part2(data: &String) -> i32 {
    println!("Part 2");
    let count = data
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()[..]
        .windows(3)
        .map(|tpl| tpl.into_iter().fold(0, |a, b| a + b))
        .collect::<Vec<i32>>()[..]
        .windows(2)
        .fold(0, |acc, tpl| acc + if tpl[1] > tpl[0] { 1 } else { 0 });
    println!("Part 2 Result: {}", count);
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_part1() {
        let data = read_to_string("test.txt").unwrap();
        let count = part1(&data);
        assert_eq!(7, count);
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("test.txt").unwrap();
        let count = part2(&data);
        assert_eq!(5, count);
    }

    #[test]
    fn real_part1() {
        let data = get_input(YEAR, DAY);
        let count = part1(&data);
        assert_eq!(1228, count);
    }

    #[test]
    fn real_part2() {
        let data = get_input(YEAR, DAY);
        let count = part2(&data);
        assert_eq!(1257, count);
    }
}
