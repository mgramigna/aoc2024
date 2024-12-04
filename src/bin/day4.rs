use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
struct Grid {
    nrows: usize,
    ncols: usize,
    grid: Vec<Vec<char>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in &self.grid {
            for col in row {
                match col {
                    'X' => s += "X ",
                    'M' => s += "M ",
                    'A' => s += "A ",
                    'S' => s += "S ",
                    _ => s += ". ",
                }
            }
            s += "\n"
        }

        write!(f, "{}", s)
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<_> = s
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<char>>())
            .collect();

        Ok(Grid {
            nrows: grid.len(),
            ncols: grid[0].len(),
            grid,
        })
    }
}

const XMAS: &str = "XMAS";

impl Grid {
    fn is_left_horizontal_xmas(&self, start_row: usize, start_col: usize) -> bool {
        if start_col < 3 {
            return false;
        }

        for i in 0..4 {
            if self.grid[start_row][start_col - i] != XMAS.chars().nth(i).unwrap() {
                return false;
            }
        }

        true
    }

    fn is_right_horizontal_xmas(&self, start_row: usize, start_col: usize) -> bool {
        if start_col >= self.ncols - 3 {
            return false;
        }

        for i in 0..4 {
            if self.grid[start_row][start_col + i] != XMAS.chars().nth(i).unwrap() {
                return false;
            }
        }

        true
    }

    fn is_vertical_down_xmas(&self, start_row: usize, start_col: usize) -> bool {
        if start_row >= self.nrows - 3 {
            return false;
        }

        for (i, row) in (start_row..=start_row + 3).enumerate() {
            if self.grid[row][start_col] != XMAS.chars().nth(i).unwrap() {
                return false;
            }
        }

        true
    }

    fn is_vertical_up_xmas(&self, start_row: usize, start_col: usize) -> bool {
        if start_row < 3 {
            return false;
        }

        for (i, row) in (start_row - 3..=start_row).rev().enumerate() {
            if self.grid[row][start_col] != XMAS.chars().nth(i).unwrap() {
                return false;
            }
        }

        true
    }

    fn is_diagonal_southeast_xmas(&self, start_row: usize, start_col: usize) -> bool {
        if start_row >= self.nrows - 3 || start_col >= self.ncols - 3 {
            return false;
        }
        for (i, row) in (start_row..=start_row + 3).enumerate() {
            if self.grid[row][start_col + i] != XMAS.chars().nth(i).unwrap() {
                return false;
            }
        }

        true
    }

    fn is_diagonal_northwest_xmas(&self, start_row: usize, start_col: usize) -> bool {
        if start_row < 3 || start_col < 3 {
            return false;
        }
        for (i, row) in (start_row - 3..=start_row).rev().enumerate() {
            if self.grid[row][start_col - i] != XMAS.chars().nth(i).unwrap() {
                return false;
            }
        }

        true
    }

    fn is_diagonal_southwest_xmas(&self, start_row: usize, start_col: usize) -> bool {
        if start_row >= self.nrows - 3 || start_col < 3 {
            return false;
        }

        for (i, row) in (start_row..=start_row + 3).enumerate() {
            if self.grid[row][start_col - i] != XMAS.chars().nth(i).unwrap() {
                return false;
            }
        }

        true
    }

    fn is_diagonal_northeast_xmas(&self, start_row: usize, start_col: usize) -> bool {
        if start_row < 3 || start_col >= self.ncols - 3 {
            return false;
        }

        for (i, row) in (start_row - 3..=start_row).rev().enumerate() {
            if self.grid[row][start_col + i] != XMAS.chars().nth(i).unwrap() {
                return false;
            }
        }

        true
    }

    fn is_x_mas(&self, start_row: usize, start_col: usize) -> bool {
        if start_row == 0
            || start_row == self.nrows - 1
            || start_col == 0
            || start_col == self.ncols - 1
        {
            return false;
        }

        let left_diag = match self.grid[start_row - 1][start_col - 1] {
            'M' => self.grid[start_row + 1][start_col + 1] == 'S',
            'S' => self.grid[start_row + 1][start_col + 1] == 'M',
            _ => false,
        };

        let right_diag = match self.grid[start_row + 1][start_col - 1] {
            'M' => self.grid[start_row - 1][start_col + 1] == 'S',
            'S' => self.grid[start_row - 1][start_col + 1] == 'M',
            _ => false,
        };

        right_diag && left_diag
    }
}

fn part_one(grid: &Grid) -> usize {
    let mut total = 0;
    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            if grid.grid[row][col] == 'X' {
                if grid.is_left_horizontal_xmas(row, col) {
                    total += 1;
                }

                if grid.is_right_horizontal_xmas(row, col) {
                    total += 1;
                }

                if grid.is_vertical_down_xmas(row, col) {
                    total += 1;
                }

                if grid.is_vertical_up_xmas(row, col) {
                    total += 1;
                }

                if grid.is_diagonal_northeast_xmas(row, col) {
                    total += 1;
                }

                if grid.is_diagonal_northwest_xmas(row, col) {
                    total += 1;
                }

                if grid.is_diagonal_southeast_xmas(row, col) {
                    total += 1;
                }

                if grid.is_diagonal_southwest_xmas(row, col) {
                    total += 1;
                }
            }
        }
    }

    total
}

fn part_two(grid: &Grid) -> usize {
    let mut total = 0;
    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            if grid.grid[row][col] == 'A' && grid.is_x_mas(row, col) {
                total += 1;
            }
        }
    }

    total
}

fn main() {
    let grid = include_str!("../../inputs/day4.txt")
        .parse::<Grid>()
        .expect("Failed to parse grid");

    println!("Part 1: {}", part_one(&grid));
    println!("Part 2: {}", part_two(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_d4() {
        let grid = r"MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX"
            .parse::<Grid>()
            .expect("Failed to parse grid");

        let res = part_one(&grid);

        assert_eq!(res, 18)
    }

    #[test]
    fn test_part_two_d4() {
        let grid = r"MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX"
            .parse::<Grid>()
            .expect("Failed to parse grid");

        let res = part_two(&grid);

        assert_eq!(res, 9)
    }
}
