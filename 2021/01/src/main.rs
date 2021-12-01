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
    let values: Vec<i32> = data
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut prev = values[0];
    let mut count = 0;
    for i in 1..values.len() {
        let curr = values[i];
        if curr > prev {
            count += 1;
        }
        prev = curr;
    }
    println!("Part 1 Result: {}", count);
    return count;
}

fn part2(data: &String) -> i32 {
    println!("Part 2");
    let values: Vec<i32> = data
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut windows: Vec<i32> = Vec::new();
    for i in 0..values.len() {
        if i >= values.len() - 2 {
            break;
        }
        let window = values[i] + values[i + 1] + values[i + 2];
        windows.push(window);
    }
    let mut prev = windows[0];
    let mut count = 0;
    for i in 1..windows.len() {
        let curr = windows[i];
        if curr > prev {
            count += 1;
        }
        prev = curr;
    }
    println!("Part 2 Result: {}", count);
    return count;
    //println!("{}", data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_part1() {
        let data = read_to_string("part1_test.txt").unwrap();
        let count = part1(&data);
        assert_eq!(7, count);
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("part2_test.txt").unwrap();
        let count = part2(&data);
        assert_eq!(5, count);
    }

    #[test]
    fn test_part1_real() {
        let data = get_input(YEAR, DAY);
        let count = part1(&data);
        assert_eq!(1228, count);
    }

    #[test]
    fn test_part2_real() {
        let data = get_input(YEAR, DAY);
        let count = part2(&data);
        assert_eq!(1257, count);
    }
}
