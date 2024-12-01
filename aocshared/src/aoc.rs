pub mod aoc {
    use aocf::Aoc;
    use itertools::Itertools;
    use std::{fs::read_to_string, ops::Range};
    /// Gets puzzle input for the provided year and day
    pub fn get_input(year: i32, day: u32) -> String {
        let mut aoc = Aoc::new().year(Some(year)).day(Some(day)).init().unwrap();
        if let Ok(i) = aoc.get_input(false) {
            i
        } else {
            panic!("No input found");
        }
    }

    /// Gets test input from the sample file
    pub fn get_test_input(year: i32, day: u32) -> String {
        let file = format!("testdata/{}/{}_{:0>2}.txt", year, year, day);
        if let Ok(i) = read_to_string(&file) {
            i
        } else {
            panic!("No input found {}", &file);
        }
    }
    /// Gets test input from the sample file
    pub fn get_test_input_part(year: i32, day: u32, part: u32) -> String {
        let file = format!("testdata/{}/{}_{:0>2}_p{}.txt", year, year, day, part);
        if let Ok(i) = read_to_string(&file) {
            i
        } else {
            panic!("No input found {}", &file);
        }
    }

    /// Gets real input from a local file
    pub fn get_real_input(year: i32, day: u32) -> String {
        let file = format!("testdata/{}/{}_{:0>2}.real", year, year, day);
        if let Ok(i) = read_to_string(&file) {
            i
        } else {
            panic!("No input found {}", &file);
        }
    }

    /// Given a string of comma separated numers, create a Vec of i32
    pub fn get_numbers_from_line(line: &str) -> Vec<i32> {
        line.trim()
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }

    /// Given a line delimited string, parse each line as a u32 and return a Vec
    pub fn get_lines_as_numbers(lines: &str) -> Vec<u32> {
        lines.lines().map(|s| s.parse::<u32>().unwrap()).collect()
    }

    /// Given a line delimited string, parse each line as a u64 and return a Vec
    pub fn get_lines_as_numbers_u64(lines: &str) -> Vec<u64> {
        lines.lines().map(|s| s.parse::<u64>().unwrap()).collect()
    }

    /// Given a line delimited string, parse each line as a i64 and return a Vec
    pub fn get_lines_as_numbers_i64(lines: &str) -> Vec<i64> {
        lines.lines().map(|s| s.parse::<i64>().unwrap()).collect()
    }

    /// Given a line delimited string, convert each line to a string and return a Vec
    pub fn get_lines_as_strs(lines: &str) -> Vec<&str> {
        lines.lines().collect()
    }

    /// Given a line delimited string, convert each line to a string and return a Vec
    /// Remove all of the empty lines
    pub fn get_lines_as_strs_rm_empty(lines: &str) -> Vec<&str> {
        lines.lines().filter(|l| !l.is_empty()).collect()
    }

    /// Given a line delimited string, convert each line to a Vec of characters
    /// and return a Vec of lines
    pub fn get_lines_as_vec_chars(lines: &str) -> Vec<Vec<char>> {
        lines.lines().map(|s| s.chars().collect()).collect()
    }

    /// Given a line delimited string, convert each line to a Vec of u32
    /// and return a Vec of lines
    pub fn get_lines_as_vec_u32(lines: &str) -> Vec<Vec<u32>> {
        lines
            .lines()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect_vec()
    }

    /// Given a line delimited string, convert each line to a Vec of bytes
    /// and return a Vec of lines
    pub fn get_lines_as_vec_bytes(lines: &str) -> Vec<Vec<u8>> {
        lines.lines().map(|s| s.bytes().collect()).collect()
    }

    pub fn strs_to_u64s(strs: Vec<&str>) -> Vec<u64> {
        strs.iter().map(|s| s.parse::<u64>().unwrap()).collect()
    }

    /// Given a line delimited string, convert each line to a Vec of u32
    /// Return a Vec of lines where each line is a Vec of u32s
    pub fn get_lines_as_grid_u32(lines: &str) -> Vec<Vec<u32>> {
        lines
            .lines()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect()
    }

    /// Given a line delimited string, convert each line to a Vec of char
    /// Return a Vec of lines where each line is a Vec of char
    pub fn get_lines_as_grid_char(lines: &str) -> Vec<Vec<char>> {
        lines.lines().map(|s| s.chars().collect()).collect()
    }

    /// Given a line delimited string, convert each line to a Vec of usize
    /// Return a Vec of lines where each line is a Vec of usize
    pub fn get_lines_as_vec_usize(lines: &str) -> Vec<Vec<usize>> {
        lines
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect()
    }

    /// Splits a slice into equal sized chunks.
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

    /// Prints a Vec of Vecs which represents a 2d grid
    pub fn print_grid<T>(grid: &Vec<Vec<T>>)
    where
        T: std::fmt::Display,
    {
        for row in grid {
            for v in row {
                print!("{}", v);
            }
            println!();
        }
    }

    /// Rotates a grid transposing x,y to y,x
    pub fn transpose<T>(grid: &Vec<Vec<T>>, default: T) -> Vec<Vec<T>>
    where
        T: std::clone::Clone,
        T: std::marker::Copy,
    {
        let mut transposed = vec![vec![default; grid.len()]; grid[0].len()];
        for (i, row) in grid.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                transposed[j][i] = *v;
            }
        }
        transposed
    }

    /// Given a point, x,y calculate all adjacent points including or excluding
    /// points diagonally adjacent
    pub fn get_adj_points(
        (y, x): (usize, usize),
        (width, height): (usize, usize),
        allow_diagonal: bool,
    ) -> Vec<(usize, usize)> {
        let adj: Vec<(i32, i32)> = if allow_diagonal {
            vec![
                (y as i32 - 1, x as i32),
                (y as i32 + 1, x as i32),
                (y as i32, x as i32 - 1),
                (y as i32, x as i32 + 1),
                (y as i32 - 1, x as i32 - 1),
                (y as i32 + 1, x as i32 + 1),
                (y as i32 - 1, x as i32 + 1),
                (y as i32 + 1, x as i32 - 1),
            ]
        } else {
            vec![
                (y as i32 - 1, x as i32),
                (y as i32 + 1, x as i32),
                (y as i32, x as i32 - 1),
                (y as i32, x as i32 + 1),
            ]
        };
        adj.into_iter()
            .filter(|(y, x)| *y >= 0 && *x >= 0 && *y < height as i32 && *x < width as i32)
            .map(|(y, x)| (y as usize, x as usize))
            .collect::<Vec<(usize, usize)>>()
    }

    /// Converts a string representing a range of type 1-3, 245-300
    /// into a std::ops::Range object of type 1..3, 245..300
    /// If inclusive is set then the resulting range is 1..=3, 245..=300
    pub fn to_range(range: &str, inclusive: bool) -> Range<usize> {
        let r = range
            .split("-")
            .map(|r| r.parse::<usize>().unwrap())
            .collect_vec();
        Range {
            start: r[0],
            end: r[1] + (inclusive as usize),
        }
    }

    /// Calculates the lowest common multiple
    pub fn lcm(first: usize, second: usize) -> usize {
        first * second / gcd(first, second)
    }

    /// Calculates the greatest common denominator
    pub fn gcd(first: usize, second: usize) -> usize {
        let mut max = first;
        let mut min = second;
        if min > max {
            let val = max;
            max = min;
            min = val;
        }

        loop {
            let res = max % min;
            if res == 0 {
                return min;
            }

            max = min;
            min = res;
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
            assert_eq!(get_lines_as_numbers(input), expected);
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
            assert_eq!(get_lines_as_vec_chars(input), expected);
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
            assert_eq!(get_lines_as_vec_u32(input), expected);
        }
        #[test]
        fn test_transpose() {
            let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            let output = transpose(&input, 0);
            let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
            assert_eq!(output, expected);
        }
        #[test]
        fn test_transpose_revert() {
            let input = vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
                vec![10, 11, 12],
            ];
            let output = transpose(&transpose(&input, 0), 0);
            assert_eq!(output, input);
        }
        #[test]
        fn test_transpose_modify() {
            let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            let mut output = transpose(&input, 0);
            output.insert(2, vec![10, 11, 12]);
            let expected = vec![
                vec![1, 4, 7],
                vec![2, 5, 8],
                vec![10, 11, 12],
                vec![3, 6, 9],
            ];
            assert_eq!(output, expected);
        }
        #[test]
        fn test_transpose_modify_reverse() {
            let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            let mut t = transpose(&input, 0);
            t.insert(2, vec![10, 11, 12]);
            let output = transpose(&t, 0);

            let expected = vec![vec![1, 2, 10, 3], vec![4, 5, 11, 6], vec![7, 8, 12, 9]];
            assert_eq!(output, expected);
        }
    }
}
