use aocshared::*;
use array_tool::vec::Intersect;
use itertools::Itertools;

const YEAR: i32 = 2022;
const DAY: u32 = 03;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i32 {
    get_lines_as_vec_chars(data)
        .iter()
        .map(|items| {
            let mut cpy = items.clone();
            cpy.drain((items.len() / 2)..).collect_vec().intersect(cpy)
        })
        .map(|d| {
            d.iter()
                .map(|a| match *a as i32 {
                    a if a >= 97 => a - 96,
                    a => a - 38,
                })
                .sum::<i32>()
        })
        .sum::<i32>()
}

fn part2(data: &String) -> i32 {
    get_lines_as_vec_chars(data)
        .chunks(3)
        .map(|chunk| chunk[0].intersect(chunk[1].intersect(chunk[2].clone())))
        .map(|d| {
            d.iter()
                .map(|a| match *a as i32 {
                    a if a >= 97 => a - 96,
                    a => a - 38,
                })
                .sum::<i32>()
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2022_03_ep1() {
        assert_eq!(157, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_03_ep2() {
        assert_eq!(70, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_03_rp1() {
        assert_eq!(7553, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_03_rp2() {
        assert_eq!(2758, part2(&get_input(YEAR, DAY)));
    }
}
