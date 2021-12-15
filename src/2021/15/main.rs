use aocshared::*;

const YEAR: i32 = 2021;
const DAY: u32 = 15;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn get_adj_points((y, x): (i32, i32), (width, height): (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (y - 1, x),
        (y + 1, x),
        (y, x - 1),
        (y, x + 1),
        // (y - 1, x - 1),
        // (y + 1, x + 1),
        // (y - 1, x + 1),
        // (y + 1, x - 1),
    ]
    .into_iter()
    .filter(|(y, x)| *y >= 0 && *x >= 0 && *y < height && *x < width)
    .collect()
}
fn part1(data: &String) -> i32 {
    let input = get_lines_as_vec_u32(data);
    print_grid(&input);

    0
}

fn part2(data: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_15_ep1() {
        assert_eq!(40, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_15_ep2() {
        assert_eq!(0, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_15_rp1() {
        assert_eq!(0, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_15_rp2() {
        assert_eq!(0, part2(&get_input(YEAR, DAY)));
    }
}
