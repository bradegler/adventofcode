use aocshared::{aoc::aoc::*, grid::grid::Grid};

const YEAR: i32 = 2023;
const DAY: u32 = 11;

const EMPTY: char = '.';
const GALAXY: char = '#';

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i64 {
    let contents = get_lines_as_grid_char(data);
    let mut gc = Grid::new(contents.clone());
    let transposed = transpose(&contents, EMPTY);

    // find empty rows
    let mut empty_rows = contents
        .iter()
        .enumerate()
        .fold(vec![], |mut empties, (y, row)| {
            if !row.contains(&GALAXY) {
                empties.push(y);
            }
            empties
        });
    empty_rows.reverse();

    // find empty columns
    let mut empty_cols = transposed
        .iter()
        .enumerate()
        .fold(vec![], |mut empties, (y, row)| {
            if !row.contains(&GALAXY) {
                empties.push(y);
            }
            empties
        });
    empty_cols.reverse();

    for index in empty_rows {
        gc.insert_row(index, vec![EMPTY; gc.width()]);
    }
    for index in empty_cols {
        gc.insert_col(index, vec![EMPTY; gc.height()]);
    }

    let galaxies = gc.find(GALAXY);
    let mut unprocessed = galaxies.clone();
    galaxies
        .iter()
        .map(|galaxy| {
            unprocessed.retain(|g| !is_same(*galaxy, *g));
            println!("{:?}", galaxy);
            let distances = unprocessed.iter().map(|g| distance(*galaxy, *g));
            distances.sum::<i64>()
        })
        .sum::<i64>()
}

fn is_same(galaxy1: (usize, usize), galaxy2: (usize, usize)) -> bool {
    galaxy1.0 == galaxy2.0 && galaxy1.1 == galaxy2.1
}
fn distance(galaxy1: (usize, usize), galaxy2: (usize, usize)) -> i64 {
    return if is_same(galaxy1, galaxy2) {
        0
    } else {
        (galaxy2.0 as i64 - galaxy1.0 as i64).abs() + (galaxy2.1 as i64 - galaxy1.1 as i64).abs()
    };
}

fn part2(data: &String) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2023_11_ep1() {
        assert_eq!(374, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_11_ep2() {
        assert_eq!(0, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_11_rp1() {
        assert_eq!(9609130, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_11_rp2() {
        assert_eq!(0, part2(&get_input(YEAR, DAY)));
    }
}
