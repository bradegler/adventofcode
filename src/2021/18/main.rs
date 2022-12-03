use aocshared::*;
use itertools::Itertools;

const YEAR: i32 = 2021;
const DAY: u32 = 18;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

#[derive(Debug, Default, Clone, Copy)]
struct SnailNumber {
    data: Option<usize>,
    parent: usize,
    children: [usize; 2],
}

struct SnailNumberBag {
    store: Vec<SnailNumber>,
}

impl SnailNumberBag {
    fn new() -> Self {
        Self {
            store: vec![SnailNumber::default()],
        }
    }
    fn new_number(&mut self) -> usize {
        self.store.push(SnailNumber::default());
        self.store.len() - 1
    }
    fn new_number_with(
        &mut self,
        data: Option<usize>,
        parent: usize,
        children: [usize; 2],
    ) -> usize {
        self.store.push(SnailNumber {
            data,
            parent,
            children,
        });
        self.store.len() - 1
    }

    fn get(&mut self, idx: usize) -> &mut SnailNumber {
        &mut self.store[idx]
    }
}

fn parse(line: &str, bag: &mut SnailNumberBag) -> usize {
    let mut id = bag.new_number();
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '[' => {
                let nid = bag.new_number();
                bag.get(nid).parent = id;
                bag.get(id).children[0] = nid;
                stack.push(id);
                id = nid;
            }
            ',' => {
                let nid = bag.new_number();
                let pid = bag.get(id).parent;
                bag.get(pid).children[1] = nid;
                bag.get(nid).parent = pid;
                id = nid;
            }
            ']' => {
                id = stack.pop().unwrap();
            }
            d => {
                bag.get(id).data = Some(d.to_digit(10).unwrap() as usize);
            }
        }
    }
    id
}
fn add(left: usize, right: usize, bag: &mut SnailNumberBag) -> usize {
    let pid = bag.new_number();
    bag.get(pid).children = [left, right];
    bag.get(left).parent = pid;
    bag.get(right).parent = pid;
    while explode(pid, 0, bag) || split(pid, bag) {}
    pid
}

fn explode(id: usize, depth: usize, bag: &mut SnailNumberBag) -> bool {
    if bag.get(id).data.is_some() {
        return false;
    }
    if explode(bag.get(id).children[0], depth + 1, bag)
        || explode(bag.get(id).children[1], depth + 1, bag)
    {
        return true;
    }
    if depth < 4 {
        return false;
    }
    for i in [0, 1] {
        let cid = bag.get(id).children[i];
        let data = bag.get(cid).data;
        let mut id = id;
        let mut pid = bag.get(id).parent;
        while pid != 0 && bag.get(pid).children[i] == id {
            id = pid;
            pid = bag.get(pid).parent;
        }
        if pid != 0 {
            id = bag.get(pid).children[i];
            while bag.get(id).data.is_none() {
                id = bag.get(id).children[1 - i];
            }
            bag.get(id).data = bag.get(id).data.map(|f| f + data.unwrap());
        }
    }
    bag.get(id).data = Some(0);
    true
}

fn split(id: usize, bag: &mut SnailNumberBag) -> bool {
    if bag.get(id).data.is_some() {
        if bag.get(id).data.unwrap() < 10 {
            return false;
        }
        let data = bag.get(id).data.unwrap();
        let ldata = ((data as f32) / 2f32).floor() as usize;
        let rdata = ((data as f32) / 2f32).ceil() as usize;
        let cid_1 = bag.new_number_with(Some(ldata), id, [0, 0]);
        let cid_2 = bag.new_number_with(Some(rdata), id, [0, 0]);
        let parent = bag.get(id).parent;
        *bag.get(id) = SnailNumber {
            data: None,
            parent,
            children: [cid_1, cid_2],
        };
        true
    } else {
        split(bag.get(id).children[0], bag) || split(bag.get(id).children[1], bag)
    }
}

fn magnitude(id: usize, bag: &mut SnailNumberBag) -> usize {
    if bag.get(id).data.is_some() {
        bag.get(id).data.unwrap()
    } else {
        3 * magnitude(bag.get(id).children[0], bag) + 2 * magnitude(bag.get(id).children[1], bag)
    }
}

fn part1(data: &String) -> usize {
    let input = get_lines_as_strs(data);
    let mut snb = SnailNumberBag::new();
    let lid = input.iter().fold(0, |lid, right| {
        if lid == 0 {
            parse(right, &mut snb)
        } else {
            add(lid, parse(right, &mut snb), &mut snb)
        }
    });
    magnitude(lid, &mut snb)
}

fn part2(data: &String) -> usize {
    let input = get_lines_as_strs(data);
    input
        .iter()
        .permutations(2)
        .map(|lines| {
            let mut snb = SnailNumberBag::new();
            magnitude(
                add(
                    parse(*lines[0], &mut snb),
                    parse(*lines[1], &mut snb),
                    &mut snb,
                ),
                &mut snb,
            )
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_18_ep1() {
        assert_eq!(4140, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_18_ep2() {
        assert_eq!(3993, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_18_rp1() {
        assert_eq!(4433, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_18_rp2() {
        assert_eq!(4559, part2(&get_input(YEAR, DAY)));
    }
}
