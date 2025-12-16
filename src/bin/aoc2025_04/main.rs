use aocshared::{
    aoc::aoc::*,
    grid::grid::{Grid, Point},
};

const YEAR: i32 = 2025;
const DAY: u32 = 04;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let grid = Grid::new(get_lines_as_grid_char(data));
    grid.into_iter()
        .map(|(y, x, c)| {
            if c == '@' {
                grid.get_adj_points((y, x), true)
                    .into_iter()
                    .filter(|(ny, nx)| grid.at((*ny, *nx)) == '@')
                    .count()
            } else {
                4
            }
        })
        .filter(|c| *c < 4)
        .count() as u64
}

fn part2(data: &String) -> u64 {
    let mut grid = Grid::new(get_lines_as_grid_char(data));
    let mut removed = 0;
    loop {
        let removals: Vec<Point<usize>> = grid
            .into_iter()
            .map(|(y, x, c)| {
                if c == '@'
                    && grid
                        .get_adj_points((y, x), true)
                        .into_iter()
                        .filter(|(ny, nx)| grid.at((*ny, *nx)) == '@')
                        .count()
                        < 4
                {
                    return Some((y, x));
                }
                None
            })
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .collect();
        if removals.len() == 0 {
            break;
        }
        removed += removals.len();
        removals.iter().for_each(|p| grid.set(*p, '.'));
    }
    removed as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2025_04_ep1() {
        assert_eq!(13, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_04_ep2() {
        assert_eq!(43, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_04_rp1() {
        assert_eq!(1502, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_04_rp2() {
        assert_eq!(9083, part2(&get_input(YEAR, DAY)));
    }
}
