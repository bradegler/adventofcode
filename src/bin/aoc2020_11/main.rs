use aocshared::aoc::aoc::*;

const YEAR: i32 = 2020;
const DAY: u32 = 11;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> usize {
    let input = get_lines_as_vec_chars(data);
    let empty = 'L';
    let occupied = '#';
    let mut grid = input;
    let height = grid.len();
    let width = grid[0].len();

    let mut occupied_seats = 0;
    let mut prev_occupied_seats = usize::MAX;
    while occupied_seats != prev_occupied_seats {
        prev_occupied_seats = occupied_seats;
        let mut mut_grid = grid.clone();
        for y in 0..height {
            for x in 0..width {
                let seat = grid[y][x];
                let neighbors = get_adj_points((y, x), (width, height), true);
                if seat == empty
                    && neighbors
                        .iter()
                        .filter(|(ny, nx)| grid[*ny][*nx] == occupied)
                        .count()
                        == 0
                {
                    mut_grid[y][x] = occupied;
                }
                if seat == occupied
                    && neighbors
                        .iter()
                        .filter(|(ny, nx)| grid[*ny][*nx] == occupied)
                        .count()
                        >= 4
                {
                    mut_grid[y][x] = empty;
                }
            }
        }
        grid = mut_grid;
        occupied_seats = grid
            .iter()
            .map(|row| row.iter().filter(|&&c| c == occupied).count())
            .sum();
    }
    occupied_seats
}

fn project_point_not_floor(
    (py, px): (usize, usize),
    (lpy, lpx): (usize, usize),
    (width, height): (usize, usize),
    grid: &Vec<Vec<char>>,
) -> Option<(usize, usize)> {
    let dy: i32 = py as i32 - lpy as i32;
    let dx: i32 = px as i32 - lpx as i32;
    let mut cy: i32 = py as i32 + dy;
    let mut cx: i32 = px as i32 + dx;
    while cy < height as i32 && cx < width as i32 && cy >= 0 && cx >= 0 {
        if grid[cy as usize][cx as usize] != '.' {
            return Some((cy as usize, cx as usize));
        }
        cy += dy as i32;
        cx += dx as i32;
    }
    return None;
}

fn part2(data: &String) -> usize {
    let input = get_lines_as_vec_chars(data);
    let empty = 'L';
    let occupied = '#';
    let floor = '.';
    let mut grid = input;
    let height = grid.len();
    let width = grid[0].len();

    let mut occupied_seats = 0;
    let mut prev_occupied_seats = usize::MAX;
    while occupied_seats != prev_occupied_seats {
        prev_occupied_seats = occupied_seats;
        let mut mut_grid = grid.clone();
        for y in 0..height {
            for x in 0..width {
                let seat = grid[y][x];
                let neighbors = get_adj_points((y, x), (width, height), true)
                    .iter()
                    .map(|(ny, nx)| {
                        if grid[*ny][*nx] == floor {
                            return project_point_not_floor(
                                (*ny, *nx),
                                (y, x),
                                (width, height),
                                &grid,
                            );
                        }
                        Some((*ny, *nx))
                    })
                    .filter(|&p| p.is_some())
                    .map(|p| p.unwrap())
                    .collect::<Vec<_>>();
                if seat == empty
                    && neighbors
                        .iter()
                        .filter(|(ny, nx)| grid[*ny][*nx] == occupied)
                        .count()
                        == 0
                {
                    mut_grid[y][x] = occupied;
                }
                if seat == occupied
                    && neighbors
                        .iter()
                        .filter(|(ny, nx)| grid[*ny][*nx] == occupied)
                        .count()
                        >= 5
                {
                    mut_grid[y][x] = empty;
                }
            }
        }
        grid = mut_grid;
        occupied_seats = grid
            .iter()
            .map(|row| row.iter().filter(|&&c| c == occupied).count())
            .sum();
        print_grid(&grid);
        println!();
    }
    occupied_seats
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2020_11_ep1() {
        assert_eq!(37, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_11_ep2() {
        assert_eq!(26, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_11_rp1() {
        assert_eq!(2164, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2020_11_rp2() {
        assert_eq!(1974, part2(&get_input(YEAR, DAY)));
    }
}
