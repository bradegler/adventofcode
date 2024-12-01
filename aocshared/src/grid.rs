pub mod grid {
    use crate::aoc::aoc::print_grid;

    pub type Point<P> = (P, P);

    pub trait Visitable<T, P> {
        fn reset_visits(&mut self);
        fn visit(&mut self, point: Point<P>) -> T;
        fn is_visited(&self, point: Point<P>) -> bool;
    }

    #[derive(Debug, Clone)]
    pub struct Grid<T> {
        contents: Vec<Vec<T>>,
        visited: Vec<Vec<bool>>,
    }

    impl Visitable<char, usize> for Grid<char> {
        fn visit(&mut self, (y, x): Point<usize>) -> char {
            self.visited[y][x] = true;
            self.at((y, x))
        }

        fn is_visited(&self, (y, x): Point<usize>) -> bool {
            self.visited[y][x]
        }

        fn reset_visits(&mut self) {
            self.visited = vec![vec![false; self.width()]; self.height()];
        }
    }

    impl<T: std::fmt::Display + std::cmp::PartialEq + Copy> Grid<T> {
        pub fn new(contents: Vec<Vec<T>>) -> Self {
            let visited = vec![vec![false; contents[0].len()]; contents.len()];
            Grid {
                contents: contents,
                visited: visited,
            }
        }

        pub fn insert_row(&mut self, index: usize, row: Vec<T>) {
            self.contents.insert(index, row);
        }

        pub fn insert_col(&mut self, index: usize, col: Vec<T>) {
            for (idx, row) in self.contents.iter_mut().enumerate() {
                row.insert(index, col[idx]);
            }
        }

        pub fn print(&self) {
            print_grid(&self.contents);
        }

        pub fn at(&self, (y, x): Point<usize>) -> T {
            self.contents[y][x]
        }

        pub fn set(&mut self, (y, x): Point<usize>, value: T) {
            self.contents[y][x] = value;
        }

        /// Get the width of the grid.
        /// This grids assumes it is of uniform shape
        pub fn width(&self) -> usize {
            self.contents[0].len()
        }

        /// Get the height of the grid.
        /// This grids assumes it is of uniform shape
        pub fn height(&self) -> usize {
            self.contents.len()
        }

        pub fn into_iter(&self) -> GridIntoIterator<T> {
            GridIntoIterator {
                grid: self.clone(),
                current: None,
            }
        }

        pub fn find(&self, target: T) -> Vec<Point<usize>> {
            self.into_iter().fold(vec![], |mut points, (y, x, c)| {
                if c == target {
                    points.push((y, x));
                }
                points
            })
        }

        pub fn in_bounds(&self, (y, x): Point<i32>) -> bool {
            y > 0 && y < self.height() as i32 && x > 0 && x < self.width() as i32
        }

        /// Given a point, x,y calculate all adjacent points including or excluding
        /// points diagonally adjacent
        pub fn get_adj_points(
            &self,
            (y, x): Point<usize>,
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
                .filter(|(y, x)| {
                    *y >= 0 && *x >= 0 && *y < self.height() as i32 && *x < self.width() as i32
                })
                .map(|(y, x)| (y as usize, x as usize))
                .collect::<Vec<(usize, usize)>>()
        }
    }

    pub struct GridIntoIterator<T> {
        grid: Grid<T>,
        current: Option<Point<usize>>,
    }

    impl<T: std::fmt::Display + std::cmp::PartialEq + Copy> Iterator for GridIntoIterator<T> {
        type Item = (usize, usize, T);

        fn next(&mut self) -> Option<Self::Item> {
            self.current = match self.current {
                Some((y, x)) => {
                    let (mut ny, mut nx) = (y, x);
                    if x + 1 == self.grid.width() {
                        ny = y + 1;
                        nx = 0;
                    } else {
                        nx = x + 1;
                    }
                    if ny == self.grid.height() {
                        None
                    } else {
                        Some((ny, nx))
                    }
                }
                None => Some((0, 0)),
            };
            match self.current {
                Some((y, x)) => Some((y, x, self.grid.at((y, x)))),
                None => None,
            }
        }
    }
}
