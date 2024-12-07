use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
struct Equation {
    target: isize,
    operands: Vec<isize>,
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target_str, list_str) = s.split_once(": ").unwrap();

        Ok(Equation {
            target: target_str.parse().unwrap(),
            operands: list_str.split(" ").map(|s| s.parse().unwrap()).collect(),
        })
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = format!("{}: ", self.target);

        for o in &self.operands {
            res += format!("{}, ", o).as_str();
        }

        f.write_str(&res)
    }
}

fn concat(a: isize, b: isize) -> isize {
    format!("{}{}", a, b).parse::<isize>().unwrap()
}

impl Equation {
    fn is_possible(&self, with_concat: bool) -> bool {
        if self.target < 0 || self.operands.is_empty() {
            return false;
        }

        if self.operands.len() == 1 {
            return self.operands[0] == self.target;
        }

        if self.operands.len() == 2 {
            return self.operands[0] + self.operands[1] == self.target
                || self.operands[0] * self.operands[1] == self.target
                || (with_concat && concat(self.operands[0], self.operands[1]) == self.target);
        }

        let op1 = self.operands[0];
        let op2 = self.operands[1];

        let mut remaining = self.operands.clone();
        remaining.drain(0..=1);

        let mut remaining_division = remaining.clone();
        remaining_division.insert(0, op1 * op2);

        let is_possible_division = Equation {
            target: self.target,
            operands: remaining_division,
        }
        .is_possible(with_concat);

        let mut remaining_addition = remaining.clone();
        remaining_addition.insert(0, op1 + op2);

        let is_possible_addition = Equation {
            target: self.target,
            operands: remaining_addition,
        }
        .is_possible(with_concat);

        let mut remaining_concat = remaining.clone();
        remaining_concat.insert(0, concat(op1, op2));
        let is_possible_concat = if with_concat {
            Equation {
                target: self.target,
                operands: remaining_concat,
            }
            .is_possible(with_concat)
        } else {
            false
        };

        is_possible_division || is_possible_addition || is_possible_concat
    }
}

fn part_one(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            line.trim()
                .parse::<Equation>()
                .expect("Unable to parse equation")
        })
        .filter_map(|e| {
            if e.is_possible(false) {
                Some(e.target)
            } else {
                None
            }
        })
        .sum()
}

fn part_two(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            line.trim()
                .parse::<Equation>()
                .expect("Unable to parse equation")
        })
        .filter_map(|e| {
            if e.is_possible(true) {
                Some(e.target)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/day7.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_possible() {
        assert!(r"190: 10 19"
            .parse::<Equation>()
            .unwrap()
            .is_possible(false));
        assert!(r"3267: 81 40 27"
            .parse::<Equation>()
            .unwrap()
            .is_possible(false));
        assert!(r"292: 11 6 16 20"
            .parse::<Equation>()
            .unwrap()
            .is_possible(false));

        assert!(!r"21037: 9 7 18 13"
            .parse::<Equation>()
            .unwrap()
            .is_possible(false));
    }

    #[test]
    fn test_is_possible_with_concat() {
        assert!(r"156: 15 6".parse::<Equation>().unwrap().is_possible(true));
        assert!(r"7290: 6 8 6 15"
            .parse::<Equation>()
            .unwrap()
            .is_possible(true));
    }

    #[test]
    fn test_part_one_d7() {
        let input = r"190: 10 19
                      3267: 81 40 27
                      83: 17 5
                      156: 15 6
                      7290: 6 8 6 15
                      161011: 16 10 13
                      192: 17 8 14
                      21037: 9 7 18 13
                      292: 11 6 16 20";

        let res = part_one(input);

        assert_eq!(res, 3749)
    }

    #[test]
    fn test_part_two_d7() {
        let input = r"190: 10 19
                      3267: 81 40 27
                      83: 17 5
                      156: 15 6
                      7290: 6 8 6 15
                      161011: 16 10 13
                      192: 17 8 14
                      21037: 9 7 18 13
                      292: 11 6 16 20";

        let res = part_two(input);

        assert_eq!(res, 11387)
    }
}
