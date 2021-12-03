use aocf::Aoc;

const YEAR: i32 = 2021;
const DAY: u32 = 4;

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
    let input = data
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let result = input.len();
    println!("Part 1 Result: {}", result);
    return result;
}

fn part2(data: &String) -> i32 {
    println!("Part 2");
    let input = data
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let result = input.len();
    println!("Part 2 Result: {}", result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_part1() {
        let data = read_to_string("test.txt").unwrap();
        let count = part1(&data);
        assert_eq!(198, count);
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("test.txt").unwrap();
        let count = part2(&data);
        assert_eq!(230, count);
    }

    #[test]
    fn real_part1() {
        let data = get_input(YEAR, DAY);
        let count = part1(&data);
        assert_eq!(3813416, count);
    }

    #[test]
    fn real_part2() {
        let data = get_input(YEAR, DAY);
        let count = part2(&data);
        assert_eq!(2990784, count);
    }
}
