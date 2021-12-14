use aocshared::*;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::HashMap;

const YEAR: i32 = 2021;
const DAY: u32 = 14;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> i32 {
    let input = get_lines_as_strs(data);
    let mut initer = input.iter();
    let template = initer.next().unwrap().chars().collect::<Vec<char>>();
    let mut recipes = HashMap::new();
    let re = Regex::new(r"(\w+) -> (\w)").unwrap();
    loop {
        let opt_recipe = initer.next();
        if opt_recipe.is_none() {
            break;
        }
        let recipe = opt_recipe.unwrap();
        if recipe.is_empty() {
            continue;
        }
        let captures = re.captures(recipe).unwrap();
        recipes.insert(captures[1].to_string(), captures[2].to_string());
    }
    let mut t = template.clone();
    for _ in 0..10 {
        let mut t1 = t
            .windows(2)
            .map(|w| {
                let lookup = w.into_iter().collect::<String>();
                let mut ret = vec![w[0]];
                ret.extend(recipes[&lookup].chars());
                //ret.push(w[1]);
                ret
            })
            .flatten()
            .collect::<Vec<char>>();
        t1.push(t[t.len() - 1]);
        t = t1;
    }
    let v = t.iter().collect::<String>();
    let counts = v.chars().into_iter().fold(vec![0; 26], |mut acc, c| {
        acc[(c as u8 - b'A') as usize] += 1;
        acc
    });
    let min = counts.iter().filter(|c| **c != 0).min().unwrap();
    let max = counts.iter().max().unwrap();
    max - min
}

fn freq_map(ins: &str) -> HashMap<(char, char), usize> {
    let mut chars = ins.chars().collect::<Vec<char>>();
    chars.sort_unstable();
    chars.dedup();
    // Create a map of all the pairs of chars
    let len = chars.len();
    (0..len)
        .map(|i| (0..len).map(|j| (chars[i], chars[j])))
        .flatten()
        .collect::<HashMap<(char, char), usize>>()
}

fn part2(data: &String) -> usize {
    let lines = get_lines_as_strs_rm_empty(data);
    let polymer = lines[0];
    let re = Regex::new(r"(\w+) -> (\w)").unwrap();
    let conversions = lines[1..]
        .iter()
        .map(|l| {
            let captures = re.captures(l).unwrap();
            (captures[1].to_string(), captures[2].to_string())
        })
        .collect::<HashMap<_, _>>();
    let mut freq = freq_map();
    for i in 0..polymer.len() - 1 {
        *freq.get_mut(&polymer[i..=i + 1]).unwrap() += 1;
    }
    for _ in 0..40 {
        let mut new_freq = freq_map();
        for (k, v) in freq.iter() {
            if let Some(c) = conversions.get(k.as_str()) {
                let k1 = format!("{}{}", k.chars().nth(0).unwrap(), c);
                let k2 = format!("{}{}", c, k.chars().nth(1).unwrap());
                *new_freq.get_mut(&k1).unwrap() += v;
                *new_freq.get_mut(&k2).unwrap() += v;
            } else {
                *new_freq.get_mut(k.as_str()).unwrap() += v;
            }
        }
        freq = new_freq;
    }

    let mut f = BTreeMap::new();
    for (k, v) in freq {
        let k = k.chars().collect::<Vec<_>>();
        *f.entry(k[0]).or_insert(0) += v;
        *f.entry(k[1]).or_insert(0) += v;
    }
    for (_, v) in &mut f {
        if (*v) & 1 == 1 {
            *v = *v / 2 + 1;
        } else {
            *v = *v / 2;
        }
    }
    let mut vals = f
        .into_iter()
        .map(|(_, v)| v)
        .filter(|v| *v != 0)
        .collect::<Vec<_>>();
    vals.sort();
    dbg!(&vals);
    vals[vals.len() - 1] - vals[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_14_ep1() {
        assert_eq!(1588, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_14_ep2() {
        assert_eq!(2188189693529, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_14_rp1() {
        assert_eq!(2967, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_14_rp2() {
        assert_eq!(, part2(&get_input(YEAR, DAY)));
    }
}
