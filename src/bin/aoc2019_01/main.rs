use aocshared::aoc::aoc::*;

const YEAR: i32 = 2019;
const DAY: u32 = 01;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i64 {
    get_lines_as_numbers_i64(data).iter().map(mass).sum::<i64>()
}

fn mass(value: &i64) -> i64 {
    (value / 3) - 2
}

fn part2(data: &String) -> i64 {
    get_lines_as_numbers_i64(data)
        .iter()
        .map(|m| {
            let mut ma = 0;
            let mut next = mass(m);
            while next > 0 {
                ma += next;
                next = mass(&next)
            }
            ma
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2019_01_ep1() {
        assert_eq!(33583, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2019_01_ep2() {
        assert_eq!(50346, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2019_01_rp1() {
        assert_eq!(3372463, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2019_01_rp2() {
        assert_eq!(5055835, part2(&get_input(YEAR, DAY)));
    }
}
