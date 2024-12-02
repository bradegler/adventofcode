use aocshared::{aoc::aoc::*, grid::grid::Grid};

const YEAR: i32 = 2023;
const DAY: u32 = 11;

const EMPTY: char = '.';
const GALAXY: char = '#';

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i, 1_000_000));
}

fn part1(data: &String) -> i64 {
    expand_and_measure(data, 2)
}

fn part2(data: &String, multiplier: usize) -> i64 {
    expand_and_measure(data, multiplier)
}

fn is_same(galaxy1: (i32, i32), galaxy2: (i32, i32)) -> bool {
    galaxy1.0 == galaxy2.0 && galaxy1.1 == galaxy2.1
}
fn distance(galaxy1: (i32, i32), galaxy2: (i32, i32)) -> i64 {
    return if is_same(galaxy1, galaxy2) {
        0
    } else {
        (galaxy2.0 as i64 - galaxy1.0 as i64).abs() + (galaxy2.1 as i64 - galaxy1.1 as i64).abs()
    };
}

fn expand_and_measure(data: &String, multiplier: usize) -> i64 {
    let contents = get_lines_as_grid_char(data);
    let gc = Grid::new(contents.clone());
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

    let empty_y = empty_rows.as_slice();
    let empty_x = empty_cols.as_slice();

    let galaxies = gc.find(GALAXY);
    println!("galaxies: {:?}", galaxies);
    let expanded = galaxies
        .iter()
        .map(|g| {
            let expand_down = empty_y.iter().fold(0, |acc, ey| {
                if *ey < g.0 {
                    acc + (1 * multiplier) - 1
                } else {
                    acc
                }
            });
            let expand_right = empty_x.iter().fold(0, |acc, ex| {
                if *ex < g.1 {
                    acc + (1 * multiplier) - 1
                } else {
                    acc
                }
            });
            ((g.0 + expand_down) as i32, (g.1 + expand_right) as i32)
        })
        .collect::<Vec<_>>();

    println!("{:?}", expanded);
    let mut unprocessed = expanded.clone();
    expanded
        .iter()
        .map(|galaxy| {
            unprocessed.retain(|g| !is_same(*galaxy, *g));
            let distances = unprocessed.iter().map(|g| distance(*galaxy, *g));
            distances.sum::<i64>()
        })
        .sum::<i64>()
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
        assert_eq!(1030, part2(&get_test_input(YEAR, DAY), 10));
    }

    #[test]
    fn t2023_11_rp1() {
        assert_eq!(9609130, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_11_rp2() {
        assert_eq!(702152204842, part2(&get_input(YEAR, DAY), 1_000_000));
    }
}
