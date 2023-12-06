#![feature(let_chains)]

use std::{fs, io, collections::HashMap};

fn input_lines(filename: &str) -> io::Result<Vec<String>> {
    fs::read_to_string(filename).map(|s| s.lines().map(ToString::to_string).collect())
}

#[derive(Clone)]
struct Grid {
    chars: Box<[char]>,
    width: usize,
    height: usize
}

impl Grid {
    fn new(input: &[String]) -> Self {
        let (width, height) = (input[0].len(), input.len());
        
        let chars: Box<[char]> = input.iter().flat_map(|line| line.chars().collect::<Vec<_>>()).collect();
        Self { chars, width, height }
    }

    fn at_pos(&self, row: usize, col: usize) -> Option<char> {
        if row >= self.width || col >= self.height {
            return None;
        }
        self.chars.get(col + row * self.width).map(char::to_owned)
    }

    fn part_numbers(&self) -> GridPartNumberIterator {
        GridPartNumberIterator { grid: self.clone(), row: 0, col: 0 }
    }

    fn bounding_iter(&self, GridPartNumberInfo{num: _, pos: (row, col), len}: &GridPartNumberInfo) -> GridBoundingIterator {
       GridBoundingIterator::new(self.clone(), (row.saturating_sub(1), col.saturating_sub(1)), (row + 2, col + len + 1))
    }

    // fn is_near_symbol(&self, info: &GridPartNumberInfo) -> bool {
    //     self.bounding_iter(info).any(|(c, _)| !(c.is_digit(10) || c == '.'))
    // }

    fn nearby_stars(&self, info: &GridPartNumberInfo) -> Vec<(usize, usize)> {
        self.bounding_iter(info).filter(|(c, _)| c == &'*').map(|(_, pos)| pos).collect()
    }
}

struct GridPartNumberInfo {
    num: u32,
    pos: (usize, usize),
    len: usize
}

struct GridBoundingIterator {
    grid: Grid,
    min: (usize, usize),
    max: (usize, usize),
    row: usize,
    col: usize,
    finished: bool
}

impl GridBoundingIterator {
    fn new(grid: Grid, min: (usize, usize), max: (usize, usize)) -> Self {
        Self { grid, min, max, row: min.0, col: min.1, finished: false }
    }
}

impl Iterator for GridBoundingIterator {
    type Item = (char, (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        while let None = self.grid.at_pos(self.row, self.col) {
            self.col += 1;
            if self.col >= self.max.1 {
                self.col = self.min.1;
                self.row += 1;
                if self.row >= self.max.0 {
                    self.finished = true;
                    return None;
                }
            }
        }

        let c = self.grid.at_pos(self.row, self.col);
        let pos = (self.row, self.col);

        self.col += 1;
        if self.col >= self.max.1 {
            self.col = self.min.1;
            self.row += 1;
            if self.row >= self.max.0 {
                self.finished = true;
            }
        }

        Some((c?, pos))
    }
}

struct GridPartNumberIterator {
    grid: Grid,
    row: usize,
    col: usize
}

impl Iterator for GridPartNumberIterator {
    type Item = GridPartNumberInfo;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let None = self.grid.at_pos(self.row, self.col) {
            return None;
        }

        while let Some(c) = self.grid.at_pos(self.row, self.col) {
            if c.is_digit(10) {
                let pos = (self.row, self.col);
                
                let mut num_chars = vec![];
                while let Some(c) = self.grid.at_pos(self.row, self.col) && c.is_digit(10) {
                    num_chars.push(c);
                    self.col += 1;
                }
                
                if self.col >= self.grid.width {
                    self.col = 0;
                    self.row += 1;
                }

                let num: u32 = num_chars.iter().collect::<String>().parse().unwrap();
                let len = num_chars.len();

                return Some(GridPartNumberInfo{ num, pos, len });
            }

            self.col += 1;
            if self.col >= self.grid.width {
                self.col = 0;
                self.row += 1;
            }
        }

        None
    }
}

fn main() -> io::Result<()> {
    let grid = Grid::new(&input_lines("resources/input.txt")?);

    // let part_numbers_sum: u32 = grid.part_numbers()
    //     .filter(|info| grid.is_near_symbol(info))
    //     .map(|info| info.num)
    //     .sum();

    // dbg!(part_numbers_sum);

    let stars_to_part_numbers: HashMap<(usize, usize), Vec<u32>> = grid.part_numbers()
        .map(|info| (info.num, grid.nearby_stars(&info)))
        .fold(HashMap::new(), |mut acc, (num, stars)| {
            for star in stars {
                match acc.get_mut(&star) {
                    Some(nums) => {
                        nums.push(num);
                    }
                    None => {
                        acc.insert(star, vec![num]);
                    }
                }
            }

            acc
        });
    
    let gear_ratio_sum: u32 = stars_to_part_numbers.iter()
        .filter(|(_, nums)| nums.len() == 2)
        .map(|(_, nums)| nums.iter().product::<u32>())
        .sum();

    dbg!(gear_ratio_sum);

    Ok(())
}
