use aocshared::*;

const YEAR: i32 = 2023;
const DAY: u32 = 05;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    solve::<Part1Seeds>(data)
}

fn part2(data: &String) -> u64 {
    let input = Input::<Part2Seeds>::parse(data);
    let important_points = important_points(&input.maps);
    let p = important_points
        .iter()
        .filter(|p| input.seeds.entries.iter().any(|s| s.contains(**p)))
        .collect::<Vec<_>>();

    p.iter()
        .map(|seed| input.mapped_value(**seed))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
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

trait Seeds {
    fn parse(input: &str) -> Self;
    fn seeds(&self) -> Vec<u64>;
}

struct Part1Seeds {
    seeds: Vec<u64>,
}

impl Seeds for Part1Seeds {
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
        (self.seed_start..self.seed_start + self.count).collect::<Vec<_>>()
    }

    fn contains(&self, p: u64) -> bool {
        p >= self.seed_start && p < self.seed_start + self.count
    }
}

#[derive(Debug)]
struct Part2Seeds {
    entries: Vec<SeedEntry>,
}

impl Seeds for Part2Seeds {
    fn parse(input: &str) -> Self {
        let seeds = input
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();

        let entries = (0..seeds.len())
            .step_by(2)
            .map(|i| {
                let seed_start = seeds[i].parse().unwrap();
                let count = seeds[i + 1].parse().unwrap();

                SeedEntry { seed_start, count }
            })
            .collect::<Vec<_>>();

        Self { entries }
    }

    fn seeds(&self) -> Vec<u64> {
        self.entries.iter().flat_map(|e| e.seeds()).collect()
    }
}

#[derive(Debug)]
struct Input<SeedType: Seeds> {
    seeds: SeedType,
    maps: Vec<Map>,
}

impl<SeedType: Seeds> Input<SeedType> {
    fn parse(input: &str) -> Self {
        let mut sections = input.split("\n\n");

        let seeds = sections.next().unwrap();
        let seeds = SeedType::parse(seeds);

        let maps = sections.map(Map::parse).collect::<Vec<_>>();

        Self { seeds, maps }
    }

    fn mapped_value(&self, mut seed: u64) -> u64 {
        for map in &self.maps {
            let entry: Option<u64> = map.entries.iter().find_map(|e| e.translate_down(seed));
            seed = entry.unwrap_or(seed);
        }

        seed
    }
}

#[derive(Debug, Clone)]
struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let _ = lines.next().unwrap();

        let entries = lines.map(MapEntry::parse).collect::<Vec<_>>();

        Self { entries }
    }
}

#[derive(Debug, Clone)]
struct MapEntry {
    dest_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

fn important_points(maps: &[Map]) -> Vec<u64> {
    let mut maps = maps.to_vec();
    maps.reverse();
    let maps = maps;

    let mut points = vec![];

    for m in maps {
        let mut translated_points = points
            .iter()
            .map(|p| {
                m.entries
                    .iter()
                    .find_map(|e| e.translate_up(*p))
                    .unwrap_or(*p)
            })
            .collect::<Vec<_>>();
        let mut new_points = m
            .entries
            .iter()
            .map(|e| e.source_range_start)
            .collect::<Vec<_>>();
        translated_points.append(&mut new_points);

        points = translated_points;
    }

    points
}

impl MapEntry {
    fn parse(l: &str) -> Self {
        let nums = l
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u64>>();

        Self {
            dest_range_start: nums[0],
            source_range_start: nums[1],
            range_length: nums[2],
        }
    }

    fn translate_down(&self, seed: u64) -> Option<u64> {
        if seed >= self.source_range_start && seed < self.source_range_start + self.range_length {
            let offset = seed - self.source_range_start;
            Some(self.dest_range_start + offset)
        } else {
            None
        }
    }

    fn translate_up(&self, seed: u64) -> Option<u64> {
        if seed >= self.dest_range_start && seed < self.dest_range_start + self.range_length {
            let offset = seed - self.dest_range_start;
            Some(self.source_range_start + offset)
        } else {
            None
        }
    }
}

fn solve<SeedType: Seeds>(sample_input: &str) -> u64 {
    let input = Input::<SeedType>::parse(sample_input);

    input
        .seeds
        .seeds()
        .iter()
        .map(|seed| input.mapped_value(*seed))
        .min()
        .unwrap()
}
