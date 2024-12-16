use std::{fmt::Display, str::FromStr};

use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum GridItem {
    Empty,
    Wall,
    Box,
    Robot,
    LeftBox,
    RightBox,
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Grid {
    nrows: usize,
    ncols: usize,
    g: Vec<Vec<GridItem>>,
    robot_pos: (usize, usize),
}

impl From<char> for GridItem {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            _ => Self::Empty,
        }
    }
}

impl From<char> for Move {
    fn from(ch: char) -> Self {
        match ch {
            '<' => Move::Left,
            '>' => Move::Right,
            '^' => Move::Up,
            'v' => Move::Down,
            _ => panic!("Invalid move"),
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::from("");
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                res += match self.g[row][col] {
                    GridItem::Empty => ".",
                    GridItem::Wall => "#",
                    GridItem::Box => "O",
                    GridItem::Robot => "@",
                    GridItem::LeftBox => "[",
                    GridItem::RightBox => "]",
                }
            }
            res += "\n";
        }

        f.write_str(&res)
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut robot_pos = (0, 0);
        let g = s
            .lines()
            .enumerate()
            .map(|(r, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .map(|(c, ch)| {
                        let item = GridItem::from(ch);

                        if item == GridItem::Robot {
                            robot_pos = (r, c);
                        }

                        item
                    })
                    .collect_vec()
            })
            .collect_vec();

        Ok(Grid {
            nrows: g.len(),
            ncols: g[0].len(),
            g,
            robot_pos,
        })
    }
}

fn parse_moves(moves_str: &str) -> Vec<Move> {
    moves_str
        .lines()
        .flat_map(|l| l.trim().chars().map(Move::from))
        .collect_vec()
}

impl GridItem {
    fn scale(&self) -> impl Iterator<Item = Self> {
        match self {
            GridItem::Empty => [GridItem::Empty, GridItem::Empty].into_iter(),
            GridItem::Wall => [GridItem::Wall, GridItem::Wall].into_iter(),
            GridItem::Box => [GridItem::LeftBox, GridItem::RightBox].into_iter(),
            GridItem::Robot => [GridItem::Robot, GridItem::Empty].into_iter(),
            GridItem::LeftBox | GridItem::RightBox => unreachable!(),
        }
    }
}

impl Grid {
    fn scale(&mut self) {
        let new_grid = self
            .g
            .iter()
            .map(|row| row.iter().flat_map(GridItem::scale).collect_vec())
            .collect_vec();

        self.g = new_grid;
        self.ncols *= 2;
        self.robot_pos.1 *= 2;
    }

    fn get_next_position(&self, (row, col): (usize, usize), m: &Move) -> Option<(usize, usize)> {
        match m {
            Move::Left => {
                if col > 0 {
                    Some((row, col - 1))
                } else {
                    None
                }
            }
            Move::Right => {
                if col < self.ncols - 1 {
                    Some((row, col + 1))
                } else {
                    None
                }
            }
            Move::Up => {
                if row > 0 {
                    Some((row - 1, col))
                } else {
                    None
                }
            }
            Move::Down => {
                if row < self.nrows - 1 {
                    Some((row + 1, col))
                } else {
                    None
                }
            }
        }
    }

    fn do_move(&mut self, m: &Move) {
        let next_robot_pos = self.get_next_position(self.robot_pos, m);

        if let Some((row, col)) = next_robot_pos {
            match self.g[row][col] {
                GridItem::Empty => {
                    self.g[self.robot_pos.0][self.robot_pos.1] = GridItem::Empty;
                    self.robot_pos = (row, col);
                }
                GridItem::Wall => {}
                GridItem::Box => {
                    let mut scan = (row, col);
                    while let Some(next_box_pos) = self.get_next_position(scan, m) {
                        match self.g[next_box_pos.0][next_box_pos.1] {
                            GridItem::Empty => {
                                self.g[self.robot_pos.0][self.robot_pos.1] = GridItem::Empty;
                                self.g[next_box_pos.0][next_box_pos.1] = GridItem::Box;
                                self.robot_pos = (row, col);
                                break;
                            }
                            GridItem::Wall => break,
                            GridItem::Box => {
                                scan = next_box_pos;
                            }

                            GridItem::Robot => unreachable!("Robot should only have one position"),
                            GridItem::LeftBox => todo!(),
                            GridItem::RightBox => todo!(),
                        }
                    }
                }
                GridItem::Robot => unreachable!("Robot should only have one position"),
                GridItem::LeftBox => todo!(),
                GridItem::RightBox => todo!(),
            }
        }
    }
}

fn part_one(input: &str) -> usize {
    let (grid_str, moves_str) = input.split_once("\n\n").unwrap();

    let mut grid = grid_str.parse::<Grid>().unwrap();
    let moves = parse_moves(moves_str);

    moves.iter().for_each(|m| grid.do_move(m));

    let mut res = 0;
    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            match grid.g[row][col] {
                GridItem::Box => {
                    res += 100 * row + col;
                }
                _ => continue,
            }
        }
    }

    res
}

fn part_two(input: &str) -> usize {
    let (grid_str, moves_str) = input.split_once("\n\n").unwrap();

    let mut grid = grid_str.parse::<Grid>().unwrap();
    let moves = parse_moves(moves_str);

    grid.scale();

    println!("{}", grid);

    // moves.iter().for_each(|m| grid.do_move(m));

    // let mut res = 0;
    // for row in 0..grid.nrows {
    //     for col in 0..grid.ncols {
    //         match grid.g[row][col] {
    //             GridItem::Box => {
    //                 res += 100 * row + col;
    //             }
    //             _ => continue,
    //         }
    //     }
    // }

    // res
    todo!()
}

fn main() {
    let input = include_str!("../../inputs/day15.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_small() {
        let input = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        let res = part_one(input);

        // lol
        assert_eq!(res, 2432)
    }

    #[test]
    fn test_part_one_large() {
        let input = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let res = part_one(input);

        assert_eq!(res, 10092)
    }

    #[test]
    fn test_part_two() {
        let input = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let res = part_two(input);

        assert_eq!(res, 9021)
    }
}
