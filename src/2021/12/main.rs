use aocshared::*;
use std::collections::HashMap;
use std::collections::HashSet;

const YEAR: i32 = 2021;
const DAY: u32 = 12;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

struct Tracker<'a> {
    seen: HashSet<&'a str>,
    doubled: Option<&'a str>,
    path: Vec<&'a str>,
}

impl<'a> Tracker<'a> {
    fn new() -> Self {
        Self {
            seen: HashSet::new(),
            doubled: None,
            path: Vec::new(),
        }
    }

    fn push(&mut self, s: &'a str) {
        if !big(s) {
            if self.seen.contains(s) {
                self.doubled = Some(s);
            } else {
                self.seen.insert(s);
            }
        }
        self.path.push(s);
    }

    fn pop(&mut self, s: &str) {
        if !big(s) {
            if self.doubled == Some(s) {
                self.doubled = None;
            } else {
                self.seen.remove(s);
            }
        }
        self.path.pop();
    }

    fn can_visit_single(&self, s: &str) -> bool {
        s != "start" && (big(s) || !self.seen.contains(s))
    }
    fn can_visit_double(&self, s: &str) -> bool {
        s != "start" && (big(s) || !self.seen.contains(s) || self.doubled == None)
    }
}

fn big(s: &str) -> bool {
    s.chars().next().unwrap().is_uppercase()
}

fn dfs<'a>(
    edges: &HashMap<&'a str, Vec<&'a str>>,
    src: &str,
    tracker: &mut Tracker<'a>,
    single: bool,
) -> i32 {
    let mut paths = 0;
    for e in edges.get(src).unwrap() {
        if *e == "end" {
            //println!("start,{},end",tracker.path.join(","));
            paths += 1;
            continue;
        }
        if single {
            if !tracker.can_visit_single(e) {
                continue;
            }
        } else {
            if !tracker.can_visit_double(e) {
                continue;
            }
        }
        tracker.push(e);
        paths += dfs(edges, e, tracker, single);
        tracker.pop(e);
    }
    paths
}

fn part1(data: &String) -> i32 {
    let edges = data.lines().map(|l| l.split_once("-").unwrap()).fold(
        HashMap::<&str, Vec<&str>>::new(),
        |mut acc, l| {
            acc.entry(l.0).or_insert(vec![]).push(l.1);
            acc.entry(l.1).or_insert(vec![]).push(l.0);
            acc
        },
    );
    println!("{:?}", edges);
    dfs(&edges, "start", &mut Tracker::new(), true)
}

fn part2(data: &String) -> i32 {
    let edges = data.lines().map(|l| l.split_once("-").unwrap()).fold(
        HashMap::<&str, Vec<&str>>::new(),
        |mut acc, l| {
            acc.entry(l.0).or_insert(vec![]).push(l.1);
            acc.entry(l.1).or_insert(vec![]).push(l.0);
            acc
        },
    );
    println!("{:?}", edges);
    dfs(&edges, "start", &mut Tracker::new(), false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_12_ep1() {
        assert_eq!(10, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_12_ep2() {
        assert_eq!(36, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_12_rp1() {
        assert_eq!(3713, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_12_rp2() {
        assert_eq!(91292, part2(&get_input(YEAR, DAY)));
    }
}
