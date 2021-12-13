use aocshared::*;
use regex::Regex;

const YEAR: i32 = 2021;
const DAY: u32 = 05;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i32 {
    let lines = get_lines_as_strs(data);
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let grid_size = 1000;
    let mut area: Vec<Vec<i32>> = vec![vec![0; grid_size as usize]; grid_size as usize];
    lines
        .iter()
        .map(|s| {
            let caps = re.captures(s).unwrap();
            let x1 = caps[1].parse::<i32>().unwrap();
            let y1 = caps[2].parse::<i32>().unwrap();
            let x2 = caps[3].parse::<i32>().unwrap();
            let y2 = caps[4].parse::<i32>().unwrap();
            (x1, y1, x2, y2)
        })
        .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
        .for_each(|(x1, y1, x2, y2)| {
            let xmod = direction(x1, x2);
            let ymod = direction(y1, y2);
            let mut x = x1;
            let mut y = y1;
            while (x != x2) || (y != y2) {
                area[y as usize][x as usize] = area[y as usize][x as usize] + 1;
                x += xmod;
                y += ymod;
            }
            area[y as usize][x as usize] = area[y as usize][x as usize] + 1;
        });
    let result: usize = area
        .iter()
        .map(|r| r.iter().filter(|c| **c > 1).count())
        .sum();
    return result.try_into().unwrap();
}

fn direction(a: i32, b: i32) -> i32 {
    if a == b {
        0
    } else if a < b {
        1
    } else {
        -1
    }
}

fn part2(data: &String) -> i32 {
    let lines = get_lines_as_strs(data);
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let grid_size = 1000;
    let mut area: Vec<Vec<i32>> = vec![vec![0; grid_size as usize]; grid_size as usize];
    lines
        .iter()
        .map(|s| {
            let caps = re.captures(s).unwrap();
            let x1 = caps[1].parse::<i32>().unwrap();
            let y1 = caps[2].parse::<i32>().unwrap();
            let x2 = caps[3].parse::<i32>().unwrap();
            let y2 = caps[4].parse::<i32>().unwrap();
            (x1, y1, x2, y2)
        })
        .for_each(|(x1, y1, x2, y2)| {
            let xmod = direction(x1, x2);
            let ymod = direction(y1, y2);
            let mut x = x1;
            let mut y = y1;
            while (x != x2) || (y != y2) {
                area[y as usize][x as usize] = area[y as usize][x as usize] + 1;
                x += xmod;
                y += ymod;
            }
            area[y as usize][x as usize] = area[y as usize][x as usize] + 1;
        });
    let result: usize = area
        .iter()
        .map(|r| r.iter().filter(|c| **c > 1).count())
        .sum();
    return result.try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_05_ep1() {
        assert_eq!(5, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_05_ep2() {
        assert_eq!(12, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_05_rp1() {
        assert_eq!(6113, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_05_rp2() {
        assert_eq!(20373, part2(&get_input(YEAR, DAY)));
    }
}
