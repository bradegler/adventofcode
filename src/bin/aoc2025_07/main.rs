use aocshared::{
    aoc::aoc::*,
    grid::grid::{Grid, Point, Visitable},
};

const YEAR: i32 = 2025;
const DAY: u32 = 07;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let mut grid = Grid::new(get_lines_as_grid_char(data));
    let mut start = 0;
    for col in 0..grid.width() {
        if grid.at((0, col)) == 'S' {
            start = col;
            break;
        }
    }
    grid.print_row(0);
    println!();

    let mut beams = vec![];
    beams.push((0, start));
    let mut splits = 0;
    for row in 1..grid.height() {
        let mut row_splits = 0;
        for b in (0..beams.len()).rev() {
            let beam = beams[b];
            if grid.at((row, beam.1)) == '.' {
                grid.set((row, beam.1), '|');
            }
            if grid.at((row, beam.1)) == '^' {
                if !grid.is_visited((row, beam.1)) {
                    grid.visit((row, beam.1));
                    grid.set((row, beam.1 - 1), '|');
                    grid.set((row, beam.1 + 1), '|');
                    beams.push((row, beam.1 - 1));
                    beams.push((row, beam.1 + 1));
                    row_splits += 1;
                }
                beams.remove(b);
            }
        }
        splits += row_splits;
        grid.print_row(row);
        print!(": +{} = {}", row_splits, splits);
        println!();
    }
    splits
}

fn part2(data: &String) -> u64 {
    let grid = Grid::new(get_lines_as_grid_char(data));
    let mut start = 0;
    for col in 0..grid.width() {
        if grid.at((0, col)) == 'S' {
            start = col;
            break;
        }
    }
    grid.print();
    let mem = Grid::new(vec![vec![0; grid.width()]; grid.height()]);
    1 + count_branches(&mut grid.clone(), &mut mem.clone(), (1, start as i32))
}

fn count_branches(grid: &mut Grid<char>, mem: &mut Grid<usize>, beam: Point<i32>) -> u64 {
    let mut count = 0;
    if grid.in_bounds(beam) {
        if grid.at((beam.0 as usize, beam.1 as usize)) == '.' {
            count = count_branches(grid, mem, (beam.0 + 1, beam.1));
        } else if grid.at((beam.0 as usize, beam.1 as usize)) == '^' {
            if !grid.is_visited((beam.0 as usize, beam.1 as usize)) {
                grid.visit((beam.0 as usize, beam.1 as usize));
                let left = count_branches(grid, mem, (beam.0, beam.1 - 1));
                let right = count_branches(grid, mem, (beam.0, beam.1 + 1));
                count = 1 + left + right;
                mem.set((beam.0 as usize, beam.1 as usize), count as usize);
            } else {
                count = mem.at((beam.0 as usize, beam.1 as usize)) as u64;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2025_07_ep1() {
        assert_eq!(21, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_07_ep2() {
        assert_eq!(40, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_07_rp1() {
        assert_eq!(1609, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_07_rp2() {
        assert_eq!(12472142047197, part2(&get_input(YEAR, DAY)));
    }
}
