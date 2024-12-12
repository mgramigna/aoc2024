use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug)]
enum GridItem {
    Blank,
    Antenna(u8),
}

#[derive(Debug)]
struct Grid {
    nrows: usize,
    ncols: usize,
    grid: Vec<Vec<GridItem>>,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|l| {
                l.trim()
                    .bytes()
                    .map(|b| match b {
                        b'.' => GridItem::Blank,
                        _ => GridItem::Antenna(b),
                    })
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
    fn get_frequency_map(&self) -> HashMap<u8, Vec<(usize, usize)>> {
        let mut frequency_map = HashMap::<u8, Vec<(usize, usize)>>::new();
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                match self.grid[row][col] {
                    GridItem::Blank => continue,
                    GridItem::Antenna(freq) => {
                        let coords = frequency_map.entry(freq).or_default();
                        coords.push((row, col));
                    }
                }
            }
        }

        frequency_map
    }

    fn get_antinode(
        &self,
        first_antenna: (usize, usize),
        second_antenna: (usize, usize),
    ) -> Option<(usize, usize)> {
        let (x1, y1) = first_antenna;
        let (x2, y2) = second_antenna;

        let newx = x2 as isize + (x2 as isize - x1 as isize);
        let newy = y2 as isize + (y2 as isize - y1 as isize);

        if newx >= 0 && newx < self.nrows as isize && newy < self.ncols as isize && newy >= 0 {
            return Some((newx as usize, newy as usize));
        }

        None
    }
}

fn part_one(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Failed to parse grid");

    let mut antinode_coords = HashSet::<(usize, usize)>::new();
    let frequency_map = grid.get_frequency_map();

    for coords in frequency_map.values() {
        for i in 0..coords.len() {
            for j in 0..i {
                let node1 = coords[i];
                let node2 = coords[j];

                if let Some((x1, y1)) = grid.get_antinode(node1, node2) {
                    antinode_coords.insert((x1, y1));
                }

                if let Some((x2, y2)) = grid.get_antinode(node2, node1) {
                    antinode_coords.insert((x2, y2));
                }
            }
        }
    }

    antinode_coords.len()
}

fn part_two(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Failed to parse grid");

    let mut antinode_coords = HashSet::<(usize, usize)>::new();
    let frequency_map = grid.get_frequency_map();

    for coords in frequency_map.values() {
        for i in 0..coords.len() {
            for j in 0..i {
                let node1 = coords[i];
                let node2 = coords[j];

                if let Some((x1, y1)) = grid.get_antinode(node1, node2) {
                    antinode_coords.insert((x1, y1));
                }

                if let Some((x2, y2)) = grid.get_antinode(node2, node1) {
                    antinode_coords.insert((x2, y2));
                }
            }
        }
    }

    antinode_coords.len()
}

fn main() {
    let input = include_str!("../../inputs/day8.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_d8() {
        let input = r"............
                      ........0...
                      .....0......
                      .......0....
                      ....0.......
                      ......A.....
                      ............
                      ............
                      ........A...
                      .........A..
                      ............
                      ............";

        let res = part_one(input);

        assert_eq!(res, 14)
    }

    #[test]
    fn test_part_two_d8() {
        let input = r"............
                      ........0...
                      .....0......
                      .......0....
                      ....0.......
                      ......A.....
                      ............
                      ............
                      ........A...
                      .........A..
                      ............
                      ............";

        let res = part_two(input);

        assert_eq!(res, 34)
    }
}
