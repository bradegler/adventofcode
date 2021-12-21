use aocshared::*;
use itertools::Itertools;
use std::collections::VecDeque;

const YEAR: i32 = 2021;
const DAY: u32 = 20;

fn main() {
    let i = get_input(YEAR, DAY);
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    println!("Part 1: [{}]", part1(&i));
    println!("Part 2: [{}]", part2(&i));
}

fn part1(data: &String) -> usize {
    let (lookup, start_pixels) = data.split_once("\n\n").unwrap();
    let lookup: Vec<_> = lookup.chars().map(|char| char == '#').collect();

    let even_void = *lookup.first().unwrap();
    let mut width = 0;
    let start_pixels: Pixels = start_pixels
        .lines()
        .map(|line| -> VecDeque<_> {
            width = line.len();
            line.chars().map(|char| char == '#').collect()
        })
        .collect();

    let mut image = Image {
        height: start_pixels.len(),
        pixels: start_pixels,
        width,
    };

    image.enhance(&lookup, false);
    image.enhance(&lookup, even_void);

    image.lit_pixels()
}

fn part2(data: &String) -> usize {
    let (lookup, start_pixels) = data.split_once("\n\n").unwrap();
    let lookup: Vec<_> = lookup.chars().map(|char| char == '#').collect();

    let even_void = *lookup.first().unwrap();
    let odd_void = *lookup.get(if even_void { 0b111111111 } else { 0 }).unwrap();

    let mut width = 0;
    let start_pixels: Pixels = start_pixels
        .lines()
        .map(|line| -> VecDeque<_> {
            width = line.len();
            line.chars().map(|char| char == '#').collect()
        })
        .collect();

    let mut image = Image {
        height: start_pixels.len(),
        pixels: start_pixels,
        width,
    };

    image.enhance(&lookup, false);
    for i in 0..=48 {
        let void = if i % 2 == 0 { even_void } else { odd_void };
        image.enhance(&lookup, void);
    }

    image.lit_pixels()
}

type Pixels = VecDeque<VecDeque<bool>>;

struct Image {
    pixels: Pixels,
    width: usize,
    height: usize,
}

impl Image {
    fn double_pad(&mut self, fill: bool) {
        self.pixels.iter_mut().for_each(|row| {
            row.push_front(fill);
            row.push_front(fill);
            row.push_back(fill);
            row.push_back(fill);
        });
        self.width += 4;

        let full_row_padding = VecDeque::from(vec![fill; self.width]);
        self.pixels.push_front(full_row_padding.to_owned());
        self.pixels.push_front(full_row_padding.to_owned());
        self.pixels.push_back(full_row_padding.to_owned());
        self.pixels.push_back(full_row_padding);

        self.height += 4;
    }

    fn enhance(&mut self, pixel_lookup: &[bool], fill: bool) {
        self.double_pad(fill);

        let new_height = self.height - 2;
        let new_width = self.width - 2;

        self.pixels = self.pixels.iter().tuple_windows::<(_, _, _)>().fold(
            VecDeque::with_capacity(new_height),
            |mut px_acc, (top, mid, bot)| {
                let mut new_mid = VecDeque::with_capacity(new_width);

                for x in 1..=new_width {
                    let p1 = (top[x - 1] as usize) << 8;
                    let p2 = (top[x] as usize) << 7;
                    let p3 = (top[x + 1] as usize) << 6;
                    let p4 = (mid[x - 1] as usize) << 5;
                    let p5 = (mid[x] as usize) << 4;
                    let p6 = (mid[x + 1] as usize) << 3;
                    let p7 = (bot[x - 1] as usize) << 2;
                    let p8 = (bot[x] as usize) << 1;
                    let p9 = bot[x + 1] as usize;

                    let pixel_sum = p1 | p2 | p3 | p4 | p5 | p6 | p7 | p8 | p9;

                    new_mid.push_back(pixel_lookup[pixel_sum]);
                }

                px_acc.push_back(new_mid);

                px_acc
            },
        );

        self.width = new_width;
        self.height = new_height;

        assert_eq!(self.pixels.len(), self.height);
    }

    fn lit_pixels(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| row.iter().filter(|p| **p).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aocshared::get_test_input;
    #[test]
    fn t2021_20_ep1() {
        assert_eq!(35, part1(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_20_ep2() {
        assert_eq!(3351, part2(&get_test_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_20_rp1() {
        assert_eq!(5419, part1(&get_input(YEAR, DAY)));
    }

    #[test]
    fn t2021_20_rp2() {
        assert_eq!(17325, part2(&get_input(YEAR, DAY)));
    }
}
