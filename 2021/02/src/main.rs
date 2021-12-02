use aocf::Aoc;

const YEAR: i32 = 2021;
const DAY: u32 = 2;

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
    let mut hor = 0;
    let mut depth = 0;
    let commands = data
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    for cmd in commands {
        let parts = cmd.split(" ").collect::<Vec<&str>>();
        let dir = parts[0];
        let amt = parts[1];
        match dir {
            "forward" => hor += amt.parse::<i32>().unwrap(),
            "down" => depth += amt.parse::<i32>().unwrap(),
            "up" => depth -= amt.parse::<i32>().unwrap(),
            _ => panic!("Unknown direction"),
        }
    }

    println!("Part 1 Result: {}", hor * depth);
    return hor * depth;
}

fn part2(data: &String) -> i32 {
    println!("Part 2");
    let mut hor = 0;
    let mut depth = 0;
    let mut aim = 0;
    let commadns = data
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    for cmd in commadns {
        let parts = cmd.split(" ").collect::<Vec<&str>>();
        let dir = parts[0];
        let amt = parts[1];
        match dir {
            "forward" => {
                hor += amt.parse::<i32>().unwrap();
                depth += amt.parse::<i32>().unwrap() * aim;
            }
            "down" => aim += amt.parse::<i32>().unwrap(),
            "up" => aim -= amt.parse::<i32>().unwrap(),
            _ => panic!("Unknown direction"),
        }
    }

    println!("Part 2 Result: {}", hor * depth);
    return hor * depth;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_part1() {
        let data = read_to_string("test.txt").unwrap();
        let count = part1(&data);
        assert_eq!(150, count);
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("test.txt").unwrap();
        let count = part2(&data);
        assert_eq!(900, count);
    }

    #[test]
    fn test_part1_real() {
        let data = get_input(YEAR, DAY);
        let count = part1(&data);
        assert_eq!(1813801, count);
    }

    #[test]
    fn test_part2_real() {
        let data = get_input(YEAR, DAY);
        let count = part2(&data);
        assert_eq!(1960569556, count);
    }
}
