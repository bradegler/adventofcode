use aocf::Aoc;
use grid::Grid;
use std::fs::read_to_string;

pub fn get_input(year: i32, day: u32) -> String {
    let mut aoc = Aoc::new().year(Some(year)).day(Some(day)).init().unwrap();
    println!(
        "Advent of Code {} - Day {}: {}",
        year,
        day,
        "title" //aoc.title.unwrap()
    );
    if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        panic!("No input found");
    }
}

pub fn get_test_input(year: i32, day: u32) -> String {
    let file = format!("testdata/{}_{:0>2}.txt", year, day);
    if let Ok(i) = read_to_string(&file) {
        i
    } else {
        panic!("No input found {}", &file);
    }
}

// Splits a slice into equal sized chunks.
pub fn chunks<T>(slice: &[T], mut size: usize) -> impl Iterator<Item = &[T]> {
    struct ChunksIterator<'a, I> {
        pub slice: &'a [I],
        pub total: usize,
        pub current: usize,
        pub chunk: usize,
    }
    impl<'a, I> Iterator for ChunksIterator<'a, I> {
        type Item = &'a [I];
        fn next(&mut self) -> Option<&'a [I]> {
            if self.current == self.total {
                None
            } else if self.slice.len() / self.chunk == 1 && self.slice.len() % self.chunk != 0 {
                self.current += 2;
                Some(self.slice)
            } else {
                self.current += 1;
                let (lhs, rhs) = self.slice.split_at(self.chunk);
                self.slice = rhs;
                Some(lhs)
            }
        }
    }
    while slice.len() % size > 1 {
        size += 1;
    }
    let mut len = slice.len() / size;
    if slice.len() % len != 0 {
        len += 1;
    }
    ChunksIterator {
        slice,
        total: len,
        chunk: size,
        current: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chunks_even_split() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut chunks = chunks(&data, 3);
        assert_eq!(chunks.next().unwrap(), &[1, 2, 3]);
        assert_eq!(chunks.next().unwrap(), &[4, 5, 6]);
        assert_eq!(chunks.next().unwrap(), &[7, 8, 9]);
    }
    #[test]
    fn test_chunks_uneven_split() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut chunks = chunks(&data, 3);
        assert_eq!(chunks.next().unwrap(), &[1, 2, 3]);
        assert_eq!(chunks.next().unwrap(), &[4, 5, 6]);
        assert_eq!(chunks.next().unwrap(), &[7, 8, 9, 10]);
    }
}

pub fn print_grid(area: &Grid<i32>, width: usize) {
    let mut idx = 0;
    area.iter().for_each(|v| {
        if *v > 0 {
            print!("{}", v);
        } else {
            print!(".");
        }
        idx += 1;
        if idx % width == 0 {
            print!("\n");
        }
    });
}
