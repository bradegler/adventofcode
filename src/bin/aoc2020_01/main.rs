use aocshared::aoc::aoc::*;

const YEAR: i32 = 2020;
const DAY: u32 = 01;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u32 {
    let entries = get_lines_as_numbers(data);
    let mut result = 0;
    for idx in 0..entries.len() {
        for jdx in idx + 1..entries.len() {
            if entries[idx] + entries[jdx] == 2020 {
                result = entries[idx] * entries[jdx];
                break;
            }
        }
        if result != 0 {
            break;
        }
    }
    result
}

fn part2(data: &String) -> u32 {
    let entries = get_lines_as_numbers(data);
    let mut result = 0;
    for idx in 0..entries.len() {
        for jdx in idx + 1..entries.len() {
            for kdx in jdx + 1..entries.len() {
                if entries[idx] + entries[jdx] + entries[kdx] == 2020 {
                    result = entries[idx] * entries[jdx] * entries[kdx];
                    break;
                }
            }
        }
        if result != 0 {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2020_01_ep1() {
        assert_eq!(514579, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_01_ep2() {
        assert_eq!(241861950, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_01_rp1() {
        assert_eq!(388075, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_01_rp2() {
        assert_eq!(293450526, part2(&get_input(YEAR, DAY)));
    }
}
