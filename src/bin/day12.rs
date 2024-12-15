use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
};

#[derive(Debug)]
struct Grid {
    nrows: usize,
    ncols: usize,
    grid: Vec<Vec<u8>>,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|l| l.trim().bytes().collect_vec())
            .collect_vec();

        Ok(Grid {
            nrows: grid.len(),
            ncols: grid[0].len(),
            grid,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::from("");
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                res += format!("{}", self.grid[row][col] as char).as_str();
            }
            res += "\n";
        }

        f.write_str(&res)
    }
}

#[derive(Debug)]
struct Area {
    perimeter: usize,
    coords: HashSet<(usize, usize)>,
}

impl Grid {
    fn get_valid_neighbors(&self, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
        let mut res = vec![];
        let area_label = self.grid[row][col];

        if row > 0 && self.grid[row - 1][col] == area_label {
            res.push((row - 1, col));
        }

        if col > 0 && self.grid[row][col - 1] == area_label {
            res.push((row, col - 1));
        }

        if row < self.nrows - 1 && self.grid[row + 1][col] == area_label {
            res.push((row + 1, col));
        }

        if col < self.ncols - 1 && self.grid[row][col + 1] == area_label {
            res.push((row, col + 1));
        }

        res
    }

    fn get_area(&self, (start_row, start_col): (usize, usize)) -> Area {
        let mut queue = VecDeque::from([(start_row, start_col)]);
        let mut area = HashSet::new();

        area.insert((start_row, start_col));

        let mut perimeter = 0;
        while let Some((row, col)) = queue.pop_front() {
            let neighbors = self.get_valid_neighbors((row, col));
            perimeter += 4 - neighbors.len();
            for neighbor in neighbors {
                if area.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        Area {
            perimeter,
            coords: area,
        }
    }
}

const DIR: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

impl Area {
    fn count_sides(&self) -> usize {
        let mut side_count = 0;
        for dir in DIR {
            let mut sides = HashSet::new();
            for pos in &self.coords {
                let tmp = (
                    (pos.0 as i32 + dir.0) as usize,
                    (pos.1 as i32 + dir.1) as usize,
                );
                if !self.coords.contains(&tmp) {
                    sides.insert(tmp);
                }
            }
            let mut remove = HashSet::new();
            for side in &sides {
                let mut tmp = (
                    (side.0 as i32 + dir.1) as usize,
                    (side.1 as i32 + dir.0) as usize,
                );
                while sides.contains(&tmp) {
                    remove.insert(tmp);
                    tmp = (
                        (tmp.0 as i32 + dir.1) as usize,
                        (tmp.1 as i32 + dir.0) as usize,
                    );
                }
            }
            side_count += sides.len() - remove.len();
        }

        side_count
    }
}

fn part_one(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Failed to parse grid");

    let mut processed = HashSet::<(usize, usize)>::new();
    let mut price = 0_usize;
    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            if processed.contains(&(row, col)) {
                continue;
            }

            let area = grid.get_area((row, col));

            price += area.coords.len() * area.perimeter;

            processed.extend(area.coords);
        }
    }

    price
}

fn part_two(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Failed to parse grid");

    let mut processed = HashSet::<(usize, usize)>::new();
    let mut price = 0_usize;
    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            if processed.contains(&(row, col)) {
                continue;
            }

            let area = grid.get_area((row, col));

            price += area.coords.len() * area.count_sides();

            processed.extend(area.coords);
        }
    }

    price
}

fn main() {
    let input = include_str!("../../inputs/day12.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_d12_a() {
        let input = r"AAAA
                      BBCD
                      BBCC
                      EEEC";
        let res = part_one(input);

        assert_eq!(res, 140)
    }

    #[test]
    fn test_part_one_d12_b() {
        let input = r"OOOOO
                      OXOXO
                      OOOOO
                      OXOXO
                      OOOOO";
        let res = part_one(input);

        assert_eq!(res, 772)
    }

    #[test]
    fn test_part_one_d12_c() {
        let input = r"RRRRIICCFF
                      RRRRIICCCF
                      VVRRRCCFFF
                      VVRCCCJFFF
                      VVVVCJJCFE
                      VVIVCCJJEE
                      VVIIICJJEE
                      MIIIIIJJEE
                      MIIISIJEEE
                      MMMISSJEEE";
        let res = part_one(input);

        assert_eq!(res, 1930)
    }

    #[test]
    fn test_part_two_d12_a() {
        let input = r"AAAA
                      BBCD
                      BBCC
                      EEEC";
        let res = part_two(input);

        assert_eq!(res, 80)
    }

    #[test]
    fn test_part_two_d12_b() {
        let input = r"EEEEE
                      EXXXX
                      EEEEE
                      EXXXX
                      EEEEE";

        let res = part_two(input);

        assert_eq!(res, 236)
    }

    #[test]
    fn test_part_two_d12_c() {
        let input = r"AAAAAA
                      AAABBA
                      AAABBA
                      ABBAAA
                      ABBAAA
                      AAAAAA";

        let res = part_two(input);

        assert_eq!(res, 368)
    }

    #[test]
    fn test_part_two_d12_d() {
        let input = r"OOOOO
                      OXOXO
                      OOOOO
                      OXOXO
                      OOOOO";
        let res = part_two(input);

        assert_eq!(res, 436)
    }
}
