use aocshared::*;
use hashbrown::HashSet;
use itertools::Itertools;

const YEAR: i32 = 2022;
const DAY: u32 = 09;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> usize {
    let lines = get_lines_as_vec_chars(data);
    let moves = lines
        .iter()
        .map(|cs| {
            let dir = cs[0];
            let amt = cs[2..].iter().collect::<String>().parse::<i32>().unwrap();
            (dir, amt)
        })
        .collect_vec();
    let mut visited = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);

    visited.insert(tail);
    for m in moves {
        let cnt = m.1;
        for _step in 1..cnt + 1 {
            match m.0 {
                'R' => head.0 += 1,
                'L' => head.0 -= 1,
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                _ => panic!(),
            }
            let hor_dist = ((head.0 - tail.0) as i32).abs();
            let ver_dist = ((head.1 - tail.1) as i32).abs();
            let same_row = head.1 == tail.1;
            let same_col = head.0 == tail.0;
            if !same_row && !same_col && (hor_dist == 2 || ver_dist == 2) {
                tail.0 += if head.0 > tail.0 { 1 } else { -1 };
                tail.1 += if head.1 > tail.1 { 1 } else { -1 };
            } else if same_row && hor_dist == 2 {
                tail.0 += if head.0 > tail.0 { 1 } else { -1 };
            } else if same_col && ver_dist == 2 {
                tail.1 += if head.1 > tail.1 { 1 } else { -1 };
            }
            visited.insert(tail);
        }
    }
    visited.len()
}

fn part2(data: &String) -> usize {
    let lines = get_lines_as_vec_chars(data);
    let moves = lines
        .iter()
        .map(|cs| {
            let dir = cs[0];
            let amt = cs[2..].iter().collect::<String>().parse::<i32>().unwrap();
            (dir, amt)
        })
        .collect_vec();
    let mut visited = HashSet::new();
    let mut knots = vec![(0, 0); 10];

    visited.insert((0, 0));
    for m in moves {
        let cnt = m.1;
        for _step in 1..cnt + 1 {
            match m.0 {
                'R' => knots[0].0 += 1,
                'L' => knots[0].0 -= 1,
                'U' => knots[0].1 += 1,
                'D' => knots[0].1 -= 1,
                _ => panic!(),
            }
            for knot in 1..10 {
                let prev = knots[knot - 1];
                let cur = knots[knot];
                let hor_dist = ((prev.0 - cur.0) as i32).abs();
                let ver_dist = ((prev.1 - cur.1) as i32).abs();
                let same_row = prev.1 == cur.1;
                let same_col = prev.0 == cur.0;
                if !same_row && !same_col && (hor_dist == 2 || ver_dist == 2) {
                    knots[knot].0 += if prev.0 > cur.0 { 1 } else { -1 };
                    knots[knot].1 += if prev.1 > cur.1 { 1 } else { -1 };
                } else if same_row && hor_dist == 2 {
                    knots[knot].0 += if prev.0 > cur.0 { 1 } else { -1 };
                } else if same_col && ver_dist == 2 {
                    knots[knot].1 += if prev.1 > cur.1 { 1 } else { -1 };
                }
            }

            visited.insert(knots[9]);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2022_09_ep1() {
        assert_eq!(13, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_09_ep2() {
        let input2 = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(36, part2(&input2.to_owned()));
    }

    #[test]
    fn t2022_09_rp1() {
        assert_eq!(6190, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2022_09_rp2() {
        assert_eq!(2516, part2(&get_input(YEAR, DAY)));
    }
}
