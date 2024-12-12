use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug)]
struct Grid {
    nrows: usize,
    ncols: usize,
    grid: Vec<Vec<u32>>,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|l| {
                l.trim()
                    .chars()
                    .map(|ch| ch.to_digit(10).expect("Grid item must be 0-9"))
                    .collect_vec()
            })
            .collect_vec();

        Ok(Grid {
            nrows: grid.len(),
            ncols: grid[0].len(),
            grid,
        })
    }
}

impl Grid {
    fn get_valid_neighbors(&self, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
        let mut res = vec![];

        // U
        if row > 0 && self.grid[row][col] + 1 == self.grid[row - 1][col] {
            res.push((row - 1, col));
        }
        // D
        if row < self.nrows - 1 && self.grid[row][col] + 1 == self.grid[row + 1][col] {
            res.push((row + 1, col));
        }
        // L
        if col > 0 && self.grid[row][col] + 1 == self.grid[row][col - 1] {
            res.push((row, col - 1));
        }
        // R
        if col < self.ncols - 1 && self.grid[row][col] + 1 == self.grid[row][col + 1] {
            res.push((row, col + 1));
        }

        res
    }

    fn get_reachable_ends_bfs(&self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut queue = VecDeque::from([(start.0, start.1)]);
        let mut seen = Vec::new();

        while let Some((row, col)) = queue.pop_front() {
            if self.grid[row][col] == 9 {
                seen.push((row, col));
                continue;
            }

            for neighbor in self.get_valid_neighbors((row, col)) {
                queue.push_back(neighbor);
            }
        }
        seen
    }
}

fn part_one(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Failed to parse grid");

    let mut res = 0;
    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            if grid.grid[row][col] != 0 {
                continue;
            }

            let seen = grid.get_reachable_ends_bfs((row, col));
            res += seen.iter().collect::<HashSet<_>>().len();
        }
    }

    res
}

fn part_two(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Failed to parse grid");

    let mut res = 0;
    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            if grid.grid[row][col] != 0 {
                continue;
            }

            let seen = grid.get_reachable_ends_bfs((row, col));
            res += seen.len();
        }
    }

    res
}

fn main() {
    let input = include_str!("../../inputs/day10.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_d10() {
        let input = r"89010123
                      78121874
                      87430965
                      96549874
                      45678903
                      32019012
                      01329801
                      10456732";

        let res = part_one(input);

        assert_eq!(res, 36)
    }

    #[test]
    fn test_part_two_d10() {
        let input = r"89010123
                      78121874
                      87430965
                      96549874
                      45678903
                      32019012
                      01329801
                      10456732";

        let res = part_two(input);

        assert_eq!(res, 81)
    }
}
