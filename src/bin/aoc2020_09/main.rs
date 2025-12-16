use aocshared::aoc::aoc::*;

const YEAR: i32 = 2020;
const DAY: u32 = 09;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i, 25));
    println!("Part 2: [{}]", part2(&i, 25));
}

fn part1(data: &String, preamble: usize) -> u64 {
    let input = get_lines_as_numbers_u64(data);

    for idx in preamble..input.len() {
        let mut found_eq = false;
        let value = input[idx];
        let subset = input[idx - preamble..idx].to_vec();
        println!("{:?}", subset);
        let ss = subset.clone();
        for x in subset {
            for y in &ss {
                println!("{} + {} = {}", x, y, x + y);
                if x + y == value {
                    found_eq = true;
                    break;
                }
            }
            if found_eq {
                break;
            }
        }
        if !found_eq {
            return value;
        }
    }
    0
}

fn part2(data: &String, preamble: usize) -> u64 {
    let target = part1(data, preamble);
    let input = get_lines_as_numbers_u64(data);
    for idx in 0..input.len() {
        for jdx in idx + 1..input.len() {
            let total = input[idx..jdx].iter().sum::<u64>();
            if total == target {
                let r = input[idx..jdx].to_vec();
                let upper = r.iter().max().unwrap();
                let lower = r.iter().min().unwrap();
                return lower + upper;
            }
        }
    }
    0u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2020_09_ep1() {
        assert_eq!(127, part1(&get_test_input(YEAR, DAY), 5));
    }

    #[test]
    fn t2020_09_ep2() {
        assert_eq!(62, part2(&get_test_input(YEAR, DAY), 5));
    }

    #[test]
    fn t2020_09_rp1() {
        assert_eq!(50047984, part1(&get_input(YEAR, DAY), 25));
    }

    #[test]
    fn t2020_09_rp2() {
        assert_eq!(5407707, part2(&get_input(YEAR, DAY), 25));
    }
}
