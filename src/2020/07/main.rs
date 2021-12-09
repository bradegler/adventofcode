use aocshared::get_input;
use regex::Regex;

const YEAR: i32 = 2020;
const DAY: u32 = 07;

fn main() {
    let i = get_input(YEAR, DAY);
    part1(&i);
    part2(&i);
}

fn part1(data: &String) -> i32 {
    println!("Part 1");
    let recontainer = Regex::new(r"([a-z ]+) bags contain (.*)\.$").unwrap();
    let recontents = Regex::new(r"(\d+) ([a-z ]+) bags?").unwrap();
    let input = data
        .lines()
        .map(|x| recontainer.captures(x).unwrap())
        .map(|c| (c[1].to_string(), c[2].to_string()))
        .map(|(b, c)| {
            if c == "no other bags.".to_string() {
                (b, vec![])
            } else {
                (
                    b,
                    recontents
                        .captures_iter(&c)
                        .map(|c| (c[1].parse::<i32>().unwrap(), c[2].to_string()))
                        .collect(),
                )
            }
        })
        .collect::<Vec<_>>();
    println!("{:?}", input);

    let result = 0;
    println!("Part 1 Result: {}", result);
    return result;
}

fn part2(data: &String) -> i32 {
    println!("Part 2");
    let result = 0;
    println!("Part 2 Result: {}", result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2020_07_ep1() {
        assert_eq!(4, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_07_ep2() {
        assert_eq!(126, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_07_rp1() {
        assert_eq!(128, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_07_rp2() {
        assert_eq!(20189, part2(&get_input(YEAR, DAY)));
    }
}
