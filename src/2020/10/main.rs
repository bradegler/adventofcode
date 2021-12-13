use aocshared::*;

const YEAR: i32 = 2020;
const DAY: u32 = 10;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let mut input = get_lines_as_numbers(data);
    input.insert(0, 0);
    input.sort();
    input.push(input[input.len() - 1] + 3);
    // Calculate the n-th discrete difference along the given axis.
    let difference = input.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    let d1 = difference.iter().filter(|&&x| x == 1).count();
    let d3 = difference.iter().filter(|&&x| x == 3).count();
    (d1 * d3) as u64
}

fn part2(data: &String) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2020_10_ep1() {
        assert_eq!(220, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_10_ep2() {
        assert_eq!(19208, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_10_rp1() {
        assert_eq!(2244, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_10_rp2() {
        assert_eq!(3947645370368, part2(&get_input(YEAR, DAY)));
    }
}
