use aocshared::aoc::aoc::*;
use itertools::Itertools;

const YEAR: i32 = 2023;
const DAY: u32 = 12;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> u64 {
    parse_report(data).iter().map(|r| r.arrangements()).sum()
}

fn part2(data: &String) -> u64 {
    0
}

fn to_spring(c: char) -> Spring {
    match c {
        '.' => Spring::Working,
        '#' => Spring::Damaged,
        '?' => Spring::Unknown,
        _ => panic!("Bad input! {:?}", c),
    }
}

fn parse_report(report: &String) -> Vec<ReportRecord> {
    let records = get_lines_as_strs_rm_empty(report);
    records
        .iter()
        .map(|record| {
            let splits = record.split(" ").collect_vec();
            let springs = splits[0];
            let groups = splits[1];
            ReportRecord {
                springs: springs.chars().map(to_spring).collect(),
                groups: groups
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_vec(),
            }
        })
        .collect()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Spring {
    Working,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReportRecord {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl ReportRecord {
    fn arrangements(&self) -> u64 {
        let spring_count = self.springs.len();
        let mut prev_counts = vec![0u64; spring_count + 2];
        prev_counts[spring_count + 1] = 1;

        for damaged_len in self.groups.iter().rev() {
            let mut possibly_damaged_run_len = 0;
            let mut nways = 0;
            let mut curr_counts = vec![0u64; spring_count + 2];

            for s in (0..spring_count).rev() {
                nways = if let Some(Spring::Damaged) = self.springs.get(s + damaged_len) {
                    0
                } else {
                    match prev_counts.get(s + damaged_len + 1) {
                        Some(ways) => nways + *ways,
                        None => 0,
                    }
                };

                curr_counts[s] = match self.springs[s] {
                    Spring::Working => {
                        possibly_damaged_run_len = 0;
                        0
                    }
                    Spring::Unknown | Spring::Damaged => {
                        possibly_damaged_run_len += 1;

                        if possibly_damaged_run_len >= *damaged_len
                            && (s == 0 || self.springs[s - 1] != Spring::Damaged)
                            && {
                                let limit = s + damaged_len;
                                limit == spring_count || self.springs[limit] != Spring::Damaged
                            }
                        {
                            nways
                        } else {
                            0
                        }
                    }
                };
            }

            prev_counts = curr_counts;
        }

        Itertools::take_while_inclusive(
            prev_counts.iter().take(spring_count).enumerate(),
            |(s, _)| self.springs[*s] != Spring::Damaged,
        )
        .map(|(_s, ways)| *ways)
        .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_arrangements() {
        let r1 = ReportRecord {
            springs: "???.###".chars().map(to_spring).collect(),
            groups: vec![1, 1, 3],
        };
        assert_eq!(1, r1.arrangements())
    }
    #[test]
    fn t2023_12_ep1() {
        assert_eq!(21, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_12_ep2() {
        assert_eq!(0, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_12_rp1() {
        assert_eq!(7922, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2023_12_rp2() {
        assert_eq!(0, part2(&get_input(YEAR, DAY)));
    }
}
