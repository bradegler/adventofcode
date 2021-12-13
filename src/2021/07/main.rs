use aocshared::*;

const YEAR: i32 = 2021;
const DAY: u32 = 07;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i32 {
    let start = get_numbers_from_line(data);
    let max = start.iter().max().unwrap();
    let result = (0..=*max)
        .fold(vec![0i32; *max as usize + 1], |mut acc, idx| {
            acc[idx as usize] = start.iter().map(|x| (x - idx).abs()).sum::<i32>();
            acc
        })
        .iter()
        .fold(
            i32::max_value(),
            |min, fc| if fc < &min { *fc } else { min },
        );
    result
}

fn part2(data: &String) -> i32 {
    let start = get_numbers_from_line(data);
    let max = start.iter().max().unwrap();
    let result = (0..=*max)
        .fold(vec![0i32; *max as usize + 1], |mut acc, idx| {
            acc[idx as usize] = start.iter().map(|x| gauss((x - idx).abs())).sum::<i32>();
            acc
        })
        .iter()
        .fold(
            i32::max_value(),
            |min, fc| if fc < &min { *fc } else { min },
        );
    result
}

fn gauss(distance: i32) -> i32 {
    (distance * (distance + 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_07_ep1() {
        assert_eq!(37, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_07_ep2() {
        assert_eq!(168, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_07_rp1() {
        assert_eq!(356922, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_07_rp2() {
        assert_eq!(100347031, part2(&get_input(YEAR, DAY)));
    }
}
