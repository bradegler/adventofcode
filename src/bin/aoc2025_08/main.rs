use aocshared::aoc::aoc::*;
use hashbrown::HashSet;
use itertools::Itertools;

const YEAR: i32 = 2025;
const DAY: u32 = 8;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i, 1000));
    println!("Part 2: [{}]", part2(&i));
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn new(values: Vec<i64>) -> Self {
        Point3D {
            x: values[0],
            y: values[1],
            z: values[2],
        }
    }
    fn distance(&self, point: &Point3D) -> f64 {
        (((point.x - self.x).pow(2) + (point.y - self.y).pow(2) + (point.z - self.z).pow(2)) as f64)
            .sqrt()
    }
}

fn part1(data: &String, circuits: usize) -> u64 {
    let points = get_lines_as_strs_rm_empty(data)
        .iter()
        .map(|s| {
            let vals = s
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec();
            Point3D::new(vals)
        })
        .collect_vec();
    let len = points.len();
    let mut distances = vec![];
    for i in 0..len {
        for j in (i + 1)..len {
            let dist = points[i].distance(&points[j]);
            distances.push((i, j, dist));
        }
    }
    distances.sort_by(|a, b| a.2.total_cmp(&b.2));
    let shorts = distances.iter().take(circuits).collect_vec();
    let mut circuits: Vec<HashSet<usize>> = vec![];
    for (left, right, _) in shorts {
        let found_left_idx = circuits.iter().position(|f| f.contains(left));
        let found_right_idx = circuits.iter().position(|f| f.contains(right));
        if found_left_idx.is_none() && found_right_idx.is_none() {
            // if we found no circuits, create a new one
            let mut set = HashSet::new();
            set.insert(*left);
            set.insert(*right);
            circuits.push(set);
        } else if found_left_idx.is_some()
            && found_right_idx.is_some()
            && !found_left_idx.eq(&found_right_idx)
        {
            // if we found two different circuits, merge them
            let left_idx = found_left_idx.unwrap();
            let right_idx = found_right_idx.unwrap();
            let lc = circuits.get(left_idx).unwrap();
            let rc = circuits.get(right_idx).unwrap();
            let mut set = HashSet::new();
            set.extend(lc);
            set.extend(rc);
            circuits.push(set);

            let mut v = vec![left_idx, right_idx];
            v.sort();
            v.iter().rev().for_each(|i| {
                circuits.remove(*i);
            });
        } else if found_left_idx.is_some() {
            // if we found one circuit, add the other point to it
            let set = circuits.get_mut(found_left_idx.unwrap()).unwrap();
            set.insert(*right);
        } else if found_right_idx.is_some() {
            let set = circuits.get_mut(found_right_idx.unwrap()).unwrap();
            set.insert(*left);
        }
    }
    circuits.sort_by(|a, b| a.len().cmp(&b.len()));
    circuits
        .iter()
        .rev()
        .take(3)
        .fold(1, |acc, c| acc * c.len() as u64)
}

fn part2(data: &String) -> u64 {
    let points = get_lines_as_strs_rm_empty(data)
        .iter()
        .map(|s| {
            let vals = s
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec();
            Point3D::new(vals)
        })
        .collect_vec();
    let len = points.len();
    let mut distances = vec![];
    for i in 0..len {
        for j in (i + 1)..len {
            let dist = points[i].distance(&points[j]);
            distances.push((i, j, dist));
        }
    }
    distances.sort_by(|a, b| a.2.total_cmp(&b.2));
    let shorts = distances.iter().collect_vec();
    let mut circuits: Vec<HashSet<usize>> = vec![];
    let mut last_left = 0;
    let mut last_right = 0;
    for (left, right, _) in shorts {
        let found_left_idx = circuits.iter().position(|f| f.contains(left));
        let found_right_idx = circuits.iter().position(|f| f.contains(right));

        if found_left_idx.is_none() && found_right_idx.is_none() {
            // if we found no circuits, create a new one
            let mut set = HashSet::new();
            set.insert(*left);
            set.insert(*right);
            circuits.push(set);

            last_left = *left;
            last_right = *right;
        } else if found_left_idx.is_some() && found_right_idx.is_some() {
            let left_idx = found_left_idx.unwrap();
            let right_idx = found_right_idx.unwrap();

            if left_idx != right_idx {
                // if we found two different circuits, merge them
                let mut set = HashSet::new();

                if left_idx > right_idx {
                    let l = circuits.remove(left_idx);
                    let r = circuits.remove(right_idx);
                    set.extend(l);
                    set.extend(r);
                } else {
                    let r = circuits.remove(right_idx);
                    let l = circuits.remove(left_idx);
                    set.extend(l);
                    set.extend(r);
                }
                circuits.push(set);

                last_left = *left;
                last_right = *right;
            }
        } else if found_left_idx.is_some() {
            let left_idx = found_left_idx.unwrap();
            // if we found one circuit, add the other point to it
            let set = circuits.get_mut(left_idx).unwrap();
            set.insert(*right);

            last_left = *left;
            last_right = *right;
        } else if found_right_idx.is_some() {
            let right_idx = found_right_idx.unwrap();

            let set = circuits.get_mut(right_idx).unwrap();
            set.insert(*left);
            last_left = *left;
            last_right = *right;
        }
    }
    let ll = &points[last_left];
    let lr = &points[last_right];
    (ll.x * lr.x) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2025_8_ep1() {
        assert_eq!(40, part1(&get_test_input(YEAR, DAY), 10));
    }

    #[test]
    fn t2025_8_ep2() {
        assert_eq!(25272, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2025_8_rp1() {
        assert_eq!(352584, part1(&get_input(YEAR, DAY), 1000));
    }

    #[test]
    fn t2025_8_rp2() {
        // too low: 570363300
        assert_eq!(9617397716, part2(&get_input(YEAR, DAY)));
    }
}
