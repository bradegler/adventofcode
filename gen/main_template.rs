use aocshared::get_input;

const YEAR: i32 = %%YEAR%%;
const DAY: u32 = %%DAY%%;

fn main() {
    let i = get_input(YEAR, DAY);
    part1(&i);
    part2(&i);
}

fn part1(data: &String) -> i32 {
    println!("Part 1");

    let result = 0;
    println!("Part 1 Result: {}", result);
    return result;
}

fn part2(data: &String) -> i32 {
    println!("Part 2");
    let result = 0;
    println!("Part 2 Result: {}", result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t%%YEAR%%_%%DAY%%_ep1() {
        assert_eq!(0, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t%%YEAR%%_%%DAY%%_ep2() {
        assert_eq!(0, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t%%YEAR%%_%%DAY%%_rp1() {
        assert_eq!(0, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t%%YEAR%%_%%DAY%%_rp2() {
        assert_eq!(0, part2(&get_input(YEAR, DAY)));
    }
}