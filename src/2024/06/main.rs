use aocshared::{
    aoc::aoc::*,
    grid::grid::{Grid, Visitable},
};

const YEAR: i32 = 2024;
const DAY: u32 = 06;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let contents = get_lines_as_grid_char(data);
    let grid = Grid::new(contents);
    guard_movement(grid).unwrap()
}

fn guard_movement(mut grid: Grid<char>) -> Option<u64> {
    grid.print();
    // there should always be only one start
    let mut prev = grid
        .into_iter()
        .find(|(_, _, c)| *c == '^')
        .map(|(y, x, _)| (y, x))
        .unwrap();
    grid.visit((prev.0, prev.1));
    let mut next = Some((prev.0 - 1, prev.1));
    let mut start_loop = None;
    while next.is_some() && grid.in_bounds((next.unwrap().0 as i32, next.unwrap().1 as i32)) {
        if grid.is_visited(next.unwrap()) {
            if next == start_loop {
                return None;
            }
            if start_loop.is_none() {
                start_loop = Some(next.unwrap());
            }
        }
        // If we hit a loop return a None condition
        if grid.at(next.unwrap()) == '#' {
            next = match (prev, next.unwrap()) {
                // north
                ((py, px), (ny, nx)) if py > ny && px == nx => Some((py, px + 1)),
                // east
                ((py, px), (ny, nx)) if py == ny && px < nx => Some((py + 1, px)),
                // west
                ((py, px), (ny, nx)) if py == ny && px > nx => Some((py - 1, px)),
                // south
                ((py, px), (ny, nx)) if py < ny && px == nx => Some((py, px - 1)),
                _ => panic!("Not moving"),
            };
        }
        grid.visit(next.unwrap());
        let new_prev = next.unwrap();
        next = grid.get_next_in_seq(prev, next.unwrap());
        prev = new_prev;
    }
    Some(
        grid.into_iter()
            .filter(|(y, x, c)| grid.is_visited((*y, *x)))
            .count() as u64,
    )
}

fn part2(data: &String) -> u64 {
    let contents = get_lines_as_grid_char(data);
    let grid = Grid::new(contents);
    grid.into_iter()
        .map(|(y, x, c)| {
            // can't overwrite start, can't overwrite obstacle
            if c == '^' || c == '#' {
                Some(0)
            } else {
                let mut g = grid.clone();
                g.set((y, x), '#');
                guard_movement(g)
            }
        })
        .filter(|n| n.is_none())
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2024_06_ep1() {
        assert_eq!(41, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_06_ep2() {
        assert_eq!(6, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2024_06_rp1() {
        assert_eq!(5312, part1(&get_input(YEAR, DAY)));
    }

    // #[test]
    // fn t2024_06_rp2() {
    //     assert_eq!(0, part2(&get_input(YEAR, DAY)));
    // }
}
