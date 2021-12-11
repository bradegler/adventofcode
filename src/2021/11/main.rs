use aocshared::get_input;
use std::collections::HashSet;

const YEAR: i32 = 2021;
const DAY: u32 = 11;

fn main() {
    let i = get_input(YEAR, DAY);
    part1(&i);
    part2(&i);
}

fn increment_point(
    (y, x): (i32, i32),
    grid: &mut Vec<Vec<u32>>,
    has_flashed: &mut HashSet<(i32, i32)>,
) -> i64 {
    if has_flashed.contains(&(y, x)) {
        return 0;
    }

    let mut flashes = 0;
    let v = grid[y as usize][x as usize];
    if v == 9 {
        // Flash
        flashes += 1;
        has_flashed.insert((y, x));
        // Increment adjacent
        flashes += increment_adj((y as i32, x as i32), grid, has_flashed);
        grid[y as usize][x as usize] = 0;
    } else {
        grid[y as usize][x as usize] += 1;
    }
    flashes
}
fn increment_adj(
    (y, x): (i32, i32),
    grid: &mut Vec<Vec<u32>>,
    has_flashed: &mut HashSet<(i32, i32)>,
) -> i64 {
    let points = vec![
        (y - 1, x),
        (y + 1, x),
        (y, x - 1),
        (y, x + 1),
        (y - 1, x - 1),
        (y + 1, x + 1),
        (y - 1, x + 1),
        (y + 1, x - 1),
    ];
    let mut flashes = 0;
    for (y, x) in points {
        if y < 0 || x < 0 || y >= grid.len() as i32 || x >= grid[y as usize].len() as i32 {
            continue;
        }
        flashes += increment_point((y, x), grid, has_flashed);
    }
    flashes
}

fn part1(data: &String) -> i32 {
    println!("Part 1");

    let mut grid = data
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut result = 0;

    for _ in 0..100 {
        let mut has_flashed: HashSet<(i32, i32)> = HashSet::new();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                result += increment_point((y as i32, x as i32), &mut grid, &mut has_flashed);
            }
        }
    }

    println!("Part 1 Result: {}", result);
    return result.try_into().unwrap();
}

fn part2(data: &String) -> i32 {
    println!("Part 2");
    let mut grid = data
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut result = 0;

    loop {
        result += 1;
        let mut has_flashed: HashSet<(i32, i32)> = HashSet::new();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                increment_point((y as i32, x as i32), &mut grid, &mut has_flashed);
            }
        }
        if has_flashed.len() == grid.len() as usize * grid[0].len() as usize {
            break;
        }
    }

    println!("Part 2 Result: {}", result);
    return result.try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_11_ep1() {
        assert_eq!(1656, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_11_ep2() {
        assert_eq!(195, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_11_rp1() {
        assert_eq!(1686, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_11_rp2() {
        assert_eq!(360, part2(&get_input(YEAR, DAY)));
    }
}
