use aocshared::aoc::aoc::*;
use itertools::Itertools;

const YEAR: i32 = 2025;
const DAY: u32 = 09;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let points: Vec<(i64, i64)> = get_lines_as_strs_rm_empty(data)
        .iter()
        .map(|s| {
            let (x, y) = s
                .split_once(",")
                .map(|s| (s.0.parse::<i64>().unwrap(), s.1.parse::<i64>().unwrap()))
                .unwrap();
            (x, y)
        })
        .collect();
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let area = area(p1, p2);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area as u64
}

fn area(p1: (i64, i64), p2: (i64, i64)) -> u64 {
    (p2.0.abs_diff(p1.0) + 1) * (p2.1.abs_diff(p1.1) + 1)
}

fn part2(data: &String) -> u64 {
    let points = get_lines_as_strs_rm_empty(data)
        .iter()
        .map(|s| {
            let (x, y) = s
                .split_once(",")
                .map(|s| (s.0.parse::<i64>().unwrap(), s.1.parse::<i64>().unwrap()))
                .unwrap();
            (x, y)
        })
        .collect_vec();

    let mut edges: Vec<((i64, i64), (i64, i64))> = vec![];
    let mut sizes = vec![];
    for i in 0..points.len() {
        let v = if i == 0 {
            vec![points[i], points[points.len() - 1]]
        } else {
            vec![points[i], points[i - 1]]
        };
        let edge = v
            .into_iter()
            .sorted()
            .collect_tuple::<((i64, i64), (i64, i64))>()
            .unwrap();
        edges.push(edge);
        for j in (i + 1)..points.len() {
            let (c1, c2) = vec![points[i], points[j]]
                .into_iter()
                .sorted()
                .collect_tuple()
                .unwrap();
            sizes.push((area(c1, c2), c1, c2));
        }
    }
    edges.sort_by(|a, b| area(a.0, a.1).cmp(&area(b.0, b.1)));
    sizes.sort();

    edges.reverse();
    sizes.reverse();

    for (size, (x1, y1), (x2, y2)) in sizes {
        let (y1, y2) = vec![y1, y2].into_iter().sorted().collect_tuple().unwrap();
        let found = edges
            .clone()
            .into_iter()
            .any(|((ex1, ey1), (ex2, ey2))| ex2 > x1 && ex1 < x2 && ey2 > y1 && ey1 < y2);
        if !found {
            return size;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2025_09_ep1() {
        assert_eq!(50, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_09_ep2() {
        assert_eq!(24, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_09_rp1() {
        assert_eq!(4777967538, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_09_rp2() {
        assert_eq!(1439894345, part2(&get_input(YEAR, DAY)));
    }
}
