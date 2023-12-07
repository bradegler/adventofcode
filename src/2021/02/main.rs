use aocshared::aoc::aoc::*;

const YEAR: i32 = 2021;
const DAY: u32 = 02;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i32 {
    let (hor, depth) = data.lines().fold((0, 0), |(hor, depth), cmd| {
        let parts = cmd.split(" ").collect::<Vec<&str>>();
        match parts[0] {
            "forward" => (hor + parts[1].parse::<i32>().unwrap(), depth),
            "down" => (hor, depth + parts[1].parse::<i32>().unwrap()),
            "up" => (hor, depth - parts[1].parse::<i32>().unwrap()),
            _ => panic!("Unknown direction"),
        }
    });
    hor * depth
}

fn part2(data: &String) -> i32 {
    let (hor, depth, _) = data.lines().fold((0, 0, 0), |(hor, depth, aim), cmd| {
        let parts = cmd.split(" ").collect::<Vec<&str>>();
        match parts[0] {
            "forward" => (
                hor + parts[1].parse::<i32>().unwrap(),
                depth + (parts[1].parse::<i32>().unwrap() * aim),
                aim,
            ),
            "down" => (hor, depth, aim + parts[1].parse::<i32>().unwrap()),
            "up" => (hor, depth, aim - parts[1].parse::<i32>().unwrap()),
            _ => panic!("Unknown direction"),
        }
    });
    return hor * depth;
}

#[cfg(test)]
mod tests {
    use super::*;
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
