use std::{ops::Range, str::FromStr};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

#[derive(Debug)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p_part, v_part) = s.split_once(" ").unwrap();

        let (_, p_str) = p_part.split_once("=").unwrap();
        let pxpy = p_str.split_once(",").unwrap();

        let (_, v_str) = v_part.split_once("=").unwrap();
        let vxvy = v_str.split_once(",").unwrap();

        Ok(Robot {
            p: (
                pxpy.0.parse::<i32>().unwrap(),
                pxpy.1.parse::<i32>().unwrap(),
            ),
            v: (
                vxvy.0.parse::<i32>().unwrap(),
                vxvy.1.parse::<i32>().unwrap(),
            ),
        })
    }
}

impl Robot {
    fn step(&mut self, width: i32, height: i32) {
        for _ in 0..self.v.0.abs() {
            let step = if self.v.0 < 0 { -1 } else { 1 };
            let mut next_x = self.p.0 + step;

            if next_x == width {
                next_x = 0;
            } else if next_x == -1 {
                next_x = width - 1
            }

            self.p.0 = next_x;
        }

        for _ in 0..self.v.1.abs() {
            let step = if self.v.1 < 0 { -1 } else { 1 };
            let mut next_y = self.p.1 + step;

            if next_y == height {
                next_y = 0;
            } else if next_y == -1 {
                next_y = height - 1;
            }

            self.p.1 = next_y;
        }
    }
}

fn range_contains(range: &Range<i32>, n: &i32) -> bool {
    range.contains(n)
}

fn part_one(input: &str, width: i32, height: i32) -> i64 {
    let mut robots = input
        .lines()
        .map(|l| l.trim().parse::<Robot>().unwrap())
        .collect_vec();

    robots.par_iter_mut().for_each(|r| {
        for _ in 0..100 {
            r.step(width, height);
        }
    });

    let mid_x = width / 2;
    let mid_y = height / 2;

    let ul_range_x = 0..mid_x;
    let ul_range_y = 0..mid_y;

    let ur_range_x = mid_x + 1..width;
    let ur_range_y = 0..mid_y;

    let dl_range_x = 0..mid_x;
    let dl_range_y = mid_y + 1..height;

    let dr_range_x = mid_x + 1..width;
    let dr_range_y = mid_y + 1..height;

    let mut ur = 0;
    let mut ul = 0;
    let mut dr = 0;
    let mut dl = 0;
    for robot in &robots {
        let (x, y) = &robot.p;

        if range_contains(&ul_range_x, x) && range_contains(&ul_range_y, y) {
            ul += 1;
        }

        if range_contains(&ur_range_x, x) && range_contains(&ur_range_y, y) {
            ur += 1;
        }

        if range_contains(&dl_range_x, x) && range_contains(&dl_range_y, y) {
            dl += 1;
        }

        if range_contains(&dr_range_x, x) && range_contains(&dr_range_y, y) {
            dr += 1;
        }
    }

    ur * ul * dr * dl
}

fn part_two(input: &str, width: i32, height: i32) -> i64 {
    todo!()
}

fn main() {
    let input = include_str!("../../inputs/day14.txt");

    println!("Part 1: {}", part_one(input, 101, 103));
    println!("Part 2: {}", part_two(input, 101, 103));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r"p=0,4 v=3,-3
                      p=6,3 v=-1,-3
                      p=10,3 v=-1,2
                      p=2,0 v=2,-1
                      p=0,0 v=1,3
                      p=3,0 v=-2,-2
                      p=7,6 v=-1,-3
                      p=3,0 v=-1,-2
                      p=9,3 v=2,3
                      p=7,3 v=-1,2
                      p=2,4 v=2,-3
                      p=9,5 v=-3,-3";

        let res = part_one(input, 11, 7);
        assert_eq!(res, 12)
    }
}
