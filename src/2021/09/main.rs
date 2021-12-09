use aocshared::get_input;
use itertools::Itertools;
use std::collections::HashSet;

const YEAR: i32 = 2021;
const DAY: u32 = 09;

fn main() {
    let i = get_input(YEAR, DAY);
    part1(&i);
    part2(&i);
}

fn part1(data: &String) -> i32 {
    println!("Part 1");
    let input = data
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut connections = vec![];
    for y in 0..input.len() {
        let row = &input[y];
        for x in 0..row.len() {
            let mut points = vec![];
            if y != 0 {
                // up
                points.push((y - 1, x));
            }
            if y != input.len() - 1 {
                // down
                points.push((y + 1, x));
            }
            if x != 0 {
                // left
                points.push((y, x - 1));
            }
            if x != row.len() - 1 {
                // right
                points.push((y, x + 1));
            }
            connections.push(((y, x), points));
        }
    }

    let result = connections
        .iter()
        .filter(|point| {
            let (y, x) = point.0;
            let value = input[y][x];
            let neighbors = &point.1;
            neighbors.iter().all(|p| input[p.0][p.1] > value)
        })
        .map(|point| {
            let (y, x) = point.0;
            let value = input[y][x] + 1;
            value
        })
        .fold(0u32, |acc, x| acc + x);

    println!("Part 1 Result: {}", result);
    return result.try_into().unwrap();
}

fn remove_component((x, y): (i32, i32), coords: &mut HashSet<(i32, i32)>) -> usize {
    if !coords.remove(&(x, y)) {
        return 0;
    }
    let n = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
    1 + n
        .iter()
        .map(|&neighbour| remove_component(neighbour, coords))
        .sum::<usize>()
}

fn part2(data: &String) -> i32 {
    println!("Part 1");
    let input = data
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect::<Vec<_>>();
    let result: usize = part2_run(&input).try_into().unwrap();
    println!("Part 1 Result: {}", result);
    return result.try_into().unwrap();
}
fn part2_run(input: &[Vec<u8>]) -> usize {
    let mut points = (0..input[0].len())
        .cartesian_product(0..input.len())
        .filter(|&(x, y)| input[y][x] != 9)
        .map(|(x, y)| (x as i32, y as i32))
        .collect::<HashSet<_>>();
    let mut cs = vec![];
    while let Some(&p) = points.iter().next() {
        cs.push(remove_component(p, &mut points));
    }
    cs.iter().sorted().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_09_ep1() {
        assert_eq!(15, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_09_ep2() {
        assert_eq!(1134, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_09_rp1() {
        assert_eq!(425, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_09_rp2() {
        assert_eq!(1135260, part2(&get_input(YEAR, DAY)));
    }
}
