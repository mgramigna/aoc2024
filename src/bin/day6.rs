use std::{collections::HashSet, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GridItem {
    Empty,
    Obstacle,
    SpecialObstacle,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Grid {
    nrows: usize,
    ncols: usize,
    grid: Vec<Vec<GridItem>>,
    guard_direction: Direction,
    guard_position: (usize, usize),
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct SeenPosition {
    position: (usize, usize),
    direction: Direction,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_direction = Direction::Up;
        let mut current_position = (0, 0);

        let grid: Vec<Vec<GridItem>> = s
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .map(|(col, ch)| match ch {
                        '.' => GridItem::Empty,
                        '#' => GridItem::Obstacle,
                        '>' => {
                            current_direction = Direction::Right;
                            current_position = (row, col);
                            GridItem::Empty
                        }
                        '<' => {
                            current_direction = Direction::Left;
                            current_position = (row, col);
                            GridItem::Empty
                        }
                        '^' => {
                            current_direction = Direction::Up;
                            current_position = (row, col);
                            GridItem::Empty
                        }
                        'v' => {
                            current_direction = Direction::Down;
                            current_position = (row, col);
                            GridItem::Empty
                        }
                        _ => panic!("Invalid grid item:  {}", ch),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(Grid {
            nrows: grid.len(),
            ncols: grid[0].len(),
            guard_direction: current_direction,
            guard_position: current_position,
            grid,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::from("");
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                if (row, col) == self.guard_position {
                    res += match self.guard_direction {
                        Direction::Left => "<",
                        Direction::Right => ">",
                        Direction::Up => "^",
                        Direction::Down => "v",
                    };
                } else {
                    res += match self.grid[row][col] {
                        GridItem::Empty => ".",
                        GridItem::Obstacle => "#",
                        GridItem::SpecialObstacle => "O",
                    };
                }
            }
            res += "\n";
        }

        f.write_str(&res)
    }
}

impl Grid {
    fn with_special_obstacle(&self, pos: (usize, usize)) -> Self {
        let mut new_grid = self.grid.clone();

        new_grid[pos.0][pos.1] = GridItem::SpecialObstacle;

        Grid {
            grid: new_grid,
            guard_position: self.guard_position,
            guard_direction: self.guard_direction,
            ncols: self.ncols,
            nrows: self.ncols,
        }
    }

    fn is_out_of_bounds(&self, pos: (usize, usize)) -> bool {
        if (pos.0 as isize) < 0
            || pos.0 == self.nrows
            || (pos.1 as isize) < 0
            || pos.1 == self.ncols
        {
            return true;
        }

        false
    }

    fn move_guard(&mut self) -> Result<(usize, usize), ()> {
        let previous_guard_position = (self.guard_position.0, self.guard_position.1);
        let next_guard_position = match self.guard_direction {
            Direction::Left => (
                self.guard_position.0,
                (self.guard_position.1 as isize - 1) as usize,
            ),
            Direction::Right => (self.guard_position.0, self.guard_position.1 + 1),
            Direction::Up => (
                (self.guard_position.0 as isize - 1) as usize,
                self.guard_position.1,
            ),
            Direction::Down => (self.guard_position.0 + 1, self.guard_position.1),
        };

        if self.is_out_of_bounds(next_guard_position) {
            return Err(());
        }

        match self.grid[next_guard_position.0][next_guard_position.1] {
            GridItem::Empty => {
                self.guard_position = next_guard_position;
            }
            GridItem::Obstacle | GridItem::SpecialObstacle => {
                self.guard_direction = match self.guard_direction {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                };

                return Ok(previous_guard_position);
            }
        };

        Ok(next_guard_position)
    }
}

fn part_one(input: &str) -> usize {
    let mut grid = input.parse::<Grid>().expect("Failed to parse grid");
    let mut seen = HashSet::from([grid.guard_position]);

    while let Ok(pos) = grid.move_guard() {
        seen.insert(pos);
    }

    seen.len()
}

fn part_two(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Failed to parse grid");

    let starting_pos = grid.guard_position;

    let mut total = 0;

    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            if (row, col) == starting_pos || grid.grid[row][col] == GridItem::Obstacle {
                continue;
            }

            let mut candidate_grid = grid.with_special_obstacle((row, col));

            let mut seen = HashSet::from([SeenPosition {
                position: candidate_grid.guard_position,
                direction: candidate_grid.guard_direction,
            }]);

            while let Ok(pos) = candidate_grid.move_guard() {
                if seen.contains(&SeenPosition {
                    position: pos,
                    direction: candidate_grid.guard_direction,
                }) {
                    total += 1;
                    break;
                }

                seen.insert(SeenPosition {
                    position: pos,
                    direction: candidate_grid.guard_direction,
                });
            }
        }
    }

    total
}

fn main() {
    let input = include_str!("../../inputs/day6.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_d6() {
        let input = r"....#.....
                      .........#
                      ..........
                      ..#.......
                      .......#..
                      ..........
                      .#..^.....
                      ........#.
                      #.........
                      ......#...";

        let res = part_one(input);

        assert_eq!(res, 41)
    }

    #[test]
    fn test_part_two_d6() {
        let input = r"....#.....
                      .........#
                      ..........
                      ..#.......
                      .......#..
                      ..........
                      .#..^.....
                      ........#.
                      #.........
                      ......#...";

        let res = part_two(input);

        assert_eq!(res, 6)
    }
}
