use aocshared::*;
use hashbrown::HashMap;

const YEAR: i32 = 2023;
const DAY: u32 = 01;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn line_value(line: &Vec<char>) -> u64 {
    let left = line.iter().position(|c| c.is_numeric()).unwrap();
    let right = line.iter().rposition(|c| c.is_numeric()).unwrap();
    let value = format!("{}{}", line[left], line[right]);
    value.parse::<u64>().unwrap()
}

fn part1(data: &String) -> u64 {
    get_lines_as_vec_chars(data)
        .iter()
        .fold(0, |acc, line| acc + line_value(line))
}

fn part2(data: &String) -> u64 {
    let words: HashMap<&str, &str> = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]);
    get_lines_as_strs(data).iter().fold(0, |acc, line| {
        let replaced1 = words
            .iter()
            .fold(line.to_string(), |acc, e| acc.replace(e.0, e.1));
        acc + line_value(&replaced1.chars().collect::<Vec<char>>())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2023_01_ep1() {
        assert_eq!(142, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_01_ep2() {
        assert_eq!(281, part2(&get_test_input_part(YEAR, DAY, 2)));
    }

    #[test]
    fn t2023_01_rp1() {
        assert_eq!(54304, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_01_rp2() {
        assert_eq!(54418, part2(&get_input(YEAR, DAY)));
    }
}
