use aocf::Aoc;
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

pub fn get_lines_as_numbers(lines: String) -> Vec<u32> {
    lines.lines().map(|s| s.parse::<u32>().unwrap()).collect()
}

pub fn get_lines_as_vec_chars(lines: String) -> Vec<Vec<char>> {
    lines.lines().map(|s| s.chars().collect()).collect()
}

pub fn get_lines_as_vec_u32(lines: String) -> Vec<Vec<u32>> {
    lines
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
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

    #[test]
    fn test_get_lines_as_numbers() {
        let input = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(get_lines_as_numbers(input.to_string()), expected);
    }
    #[test]
    fn test_get_lines_as_vec_char() {
        let input = "12\n23\n34\n45\n56\n67\n78\n89";
        let expected = vec![
            vec!['1', '2'],
            vec!['2', '3'],
            vec!['3', '4'],
            vec!['4', '5'],
            vec!['5', '6'],
            vec!['6', '7'],
            vec!['7', '8'],
            vec!['8', '9'],
        ];
        assert_eq!(get_lines_as_vec_chars(input.to_string()), expected);
    }
    #[test]
    fn test_get_lines_as_vec_u32() {
        let input = "12\n23\n34\n45\n56\n67\n78\n89";
        let expected = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7],
            vec![7, 8],
            vec![8, 9],
        ];
        assert_eq!(get_lines_as_vec_u32(input.to_string()), expected);
    }
}
