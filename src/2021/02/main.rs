use aocshared::get_input;

const YEAR: i32 = 2021;
const DAY: u32 = 2;

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
    let commands = data
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    for cmd in commands {
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
    use aocshared::get_test_input;
    #[test]
    fn t2021_02_ep1() {
        assert_eq!(150, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_02_ep2() {
        assert_eq!(900, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_02_rp1() {
        assert_eq!(1813801, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_02_rp2() {
        assert_eq!(1960569556, part2(&get_input(YEAR, DAY)));
    }
}
