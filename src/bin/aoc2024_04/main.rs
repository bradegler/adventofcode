use aocshared::{aoc::aoc::*, grid::grid::Grid};

const YEAR: i32 = 2024;
const DAY: u32 = 04;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let content = get_lines_as_grid_char(data);
    let grid = Grid::new(content);
    grid.into_iter()
        .filter(|(_, _, c)| *c == 'X')
        .map(|(y, x, _)| (y, x))
        .map(|x_pt| {
            grid.get_adj_points(x_pt, true)
                .iter()
                .filter(|m_pt| grid.at(**m_pt) == 'M')
                .map(|m_pt| {
                    grid.get_next_in_seq(x_pt, *m_pt)
                        .filter(|pt| grid.at(*pt) == 'A')
                        .and_then(|a_pt| {
                            grid.get_next_in_seq(*m_pt, a_pt)
                                .filter(|pt| grid.at(*pt) == 'S')
                                .map(|_| 1)
                        })
                        .unwrap_or(0)
                })
                .sum::<u64>()
        })
        .sum()
}

fn part2(data: &String) -> u64 {
    let content = get_lines_as_grid_char(data);
    let grid = Grid::new(content);
    grid.into_iter()
        .filter(|(ay, ax, c)| {
            *c == 'A' && *ay != 0 && *ay != grid.height() - 1 && *ax != 0 && *ax != grid.width() - 1
        })
        .map(|(ay, ax, _)| {
            match (
                grid.at((ay - 1, ax - 1)),
                grid.at((ay - 1, ax + 1)),
                grid.at((ay + 1, ax - 1)),
                grid.at((ay + 1, ax + 1)),
            ) {
                (tl, tr, bl, br) if tl == 'M' && br == 'S' && tr == 'M' && bl == 'S' => 1,
                (tl, tr, bl, br) if tl == 'M' && br == 'S' && tr == 'S' && bl == 'M' => 1,
                (tl, tr, bl, br) if tl == 'S' && br == 'M' && tr == 'S' && bl == 'M' => 1,
                (tl, tr, bl, br) if tl == 'S' && br == 'M' && tr == 'M' && bl == 'S' => 1,
                _ => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2024_04_ep1() {
        assert_eq!(18, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_04_ep2() {
        assert_eq!(9, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_04_rp1() {
        assert_eq!(2557, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_04_rp2() {
        assert_eq!(1854, part2(&get_input(YEAR, DAY)));
    }
}
