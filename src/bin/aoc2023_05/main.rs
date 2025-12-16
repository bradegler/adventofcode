use aocshared::aoc::aoc::*;
use itertools::Itertools;

const YEAR: i32 = 2023;
const DAY: u32 = 05;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    let input = Input::<Seeds1>::parse(data);

    input
        .seeds
        .seeds()
        .iter()
        .map(|seed| input.mapped_value(*seed))
        .min()
        .unwrap()
}

fn part2(data: &String) -> u64 {
    let input = Input::<Seeds2>::parse(data);
    let important_points = important_points(&input.maps);
    let p = important_points
        .iter()
        .filter(|p| input.seeds.entries.iter().any(|s| s.contains(**p)))
        .collect_vec();

    p.iter()
        .map(|seed| input.mapped_value(**seed))
        .min()
        .unwrap()
}

trait Seeds {
    fn parse(input: &str) -> Self;
    fn seeds(&self) -> Vec<u64>;
}

struct Seeds1 {
    seeds: Vec<u64>,
}

impl Seeds for Seeds1 {
    fn parse(input: &str) -> Self {
        let seeds = input
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        Self { seeds }
    }

    fn seeds(&self) -> Vec<u64> {
        self.seeds.clone()
    }
}

#[derive(Debug)]
struct SeedEntry {
    seed_start: u64,
    count: u64,
}

impl SeedEntry {
    fn seeds(&self) -> Vec<u64> {
        (self.seed_start..self.seed_start + self.count).collect_vec()
    }

    fn contains(&self, p: u64) -> bool {
        p >= self.seed_start && p < self.seed_start + self.count
    }
}

#[derive(Debug)]
struct Seeds2 {
    entries: Vec<SeedEntry>,
}

impl Seeds for Seeds2 {
    fn parse(input: &str) -> Self {
        let seeds = input
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .collect_vec();

        let entries = (0..seeds.len())
            .step_by(2)
            .map(|i| {
                let seed_start = seeds[i].parse().unwrap();
                let count = seeds[i + 1].parse().unwrap();

                SeedEntry { seed_start, count }
            })
            .collect_vec();

        Self { entries }
    }

    fn seeds(&self) -> Vec<u64> {
        self.entries.iter().flat_map(|e| e.seeds()).collect()
    }
}

#[derive(Debug)]
struct Input<SeedType: Seeds> {
    seeds: SeedType,
    maps: Vec<Range>,
}

impl<SeedType: Seeds> Input<SeedType> {
    fn parse(input: &str) -> Self {
        let mut sections = input.split("\n\n");

        let seeds = sections.next().unwrap();
        let seeds = SeedType::parse(seeds);

        let maps = sections.map(Range::parse).collect_vec();

        Self { seeds, maps }
    }

    fn mapped_value(&self, seed: u64) -> u64 {
        *&self.maps.iter().fold(seed, |seed, range| {
            range
                .entries
                .iter()
                .find_map(|e| e.source_dest_map(seed))
                .unwrap_or(seed)
        })
    }
}

#[derive(Debug, Clone)]
struct Range {
    entries: Vec<RangeEntry>,
}

impl Range {
    fn parse(input: &str) -> Self {
        Self {
            entries: input.lines().skip(1).map(RangeEntry::parse).collect_vec(),
        }
    }
}

#[derive(Debug, Clone)]
struct RangeEntry {
    dest_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

fn important_points(range: &[Range]) -> Vec<u64> {
    let range = range.to_vec();
    let mut range = range;
    range.reverse();
    let range = range;

    range.iter().fold(vec![], |points, m| {
        let mut translated_points = points
            .iter()
            .map(|p| {
                m.entries
                    .iter()
                    .find_map(|e| e.dest_source_map(*p))
                    .unwrap_or(*p)
            })
            .collect_vec();
        let mut new_points = m.entries.iter().map(|e| e.source_range_start).collect_vec();
        translated_points.append(&mut new_points);
        translated_points
    })
}

impl RangeEntry {
    fn parse(l: &str) -> Self {
        let nums = l
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect_vec();

        Self {
            dest_range_start: nums[0],
            source_range_start: nums[1],
            range_length: nums[2],
        }
    }

    fn source_dest_map(&self, seed: u64) -> Option<u64> {
        if seed >= self.source_range_start && seed < self.source_range_start + self.range_length {
            Some(self.dest_range_start + seed - self.source_range_start)
        } else {
            None
        }
    }

    fn dest_source_map(&self, seed: u64) -> Option<u64> {
        if seed >= self.dest_range_start && seed < self.dest_range_start + self.range_length {
            Some(self.source_range_start + seed - self.dest_range_start)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2023_05_ep1() {
        assert_eq!(35, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_05_ep2() {
        assert_eq!(46, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_05_rp1() {
        assert_eq!(178159714, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_05_rp2() {
        assert_eq!(100165128, part2(&get_input(YEAR, DAY)));
    }
}
