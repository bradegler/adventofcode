
use aocf::Aoc;

/* 
Advent of Code - Day 1



*/

fn main() {

    println!("Advent of Code - Day 1");

    let mut aoc = Aoc::new()
    .year(Some(2020))
    .day(Some(1))
    .init()
    .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false);
    if let Ok(i) = input {
        println!("{}", i);
        part1(&i);
        part2(&i);
    }
}

fn part1(data : &String) {
    println!("Part 1");
    println!("{}", data);
}

fn part2(data : &String) {
    println!("Part 2");
    println!("{}", data);
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;
    #[test]
    fn test_part1() {
        let data = read_to_string("part1_test.txt").unwrap();
        part1(&data);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("part2_test.txt").unwrap();
        part2(&data);
        assert_eq!(2 + 2, 4);
    }
}