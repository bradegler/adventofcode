use aocshared::*;

const YEAR: i32 = 2020;
const DAY: u32 = 03;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

const TREE: char = '#';

fn count_trees(landscape: &Vec<Vec<char>>, slope: (usize, usize)) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    while y < landscape.len() {
        if landscape[y][x] == TREE {
            trees += 1;
        }
        x = (x + slope.0) % landscape[0].len();
        y += slope.1;
    }
    trees
}

fn part1(data: &String) -> i32 {
    let landscape = get_lines_as_vec_chars(data);
    count_trees(&landscape, (3, 1))
}

fn part2(data: &String) -> i32 {
    let landscape = get_lines_as_vec_chars(data);
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .fold(1, |acc, slope| acc * count_trees(&landscape, *slope))
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2020_03_ep1() {
        assert_eq!(7, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_03_ep2() {
        assert_eq!(336, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_03_rp1() {
        assert_eq!(254, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_03_rp2() {
        assert_eq!(1666768320, part2(&get_input(YEAR, DAY)));
    }
}
