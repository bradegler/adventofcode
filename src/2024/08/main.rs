use aocshared::{aoc::aoc::*, grid::grid::Grid};
use itertools::Itertools;

const YEAR: i32 = 2024;
const DAY: u32 = 08;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let content = Grid::new(get_lines_as_grid_char(data));
    let antinodes = vec![vec![0; content.width()]; content.height()];

    let nodes = content
        .into_iter()
        .filter(|(_, _, v)| *v != '.')
        .collect_vec();

    for node in nodes {
        println!("{:?}", node);
    }
    Grid::new(antinodes).into_iter().map(|(_, _, i)| i).sum()
}

fn part2(data: &String) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2024_08_ep1() {
        assert_eq!(14, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_08_ep2() {
        assert_eq!(0, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_08_rp1() {
        assert_eq!(0, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_08_rp2() {
        assert_eq!(0, part2(&get_input(YEAR, DAY)));
    }
}
