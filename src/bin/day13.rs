use std::str::FromStr;

#[derive(Debug)]
struct ClawMachine {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    px: i64,
    py: i64,
}

impl FromStr for ClawMachine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let button_a_str = s.lines().nth(0).unwrap().trim();
        let button_b_str = s.lines().nth(1).unwrap().trim();
        let prize_str = s.lines().nth(2).unwrap().trim();

        let a_replaced = button_a_str.replace("Button A: ", "");
        let (a_dx_str, a_dy_str) = a_replaced.split_once(", ").unwrap();

        let button_a_dx = a_dx_str.split_once("+").unwrap().1.parse::<i64>().unwrap();

        let button_a_dy = a_dy_str.split_once("+").unwrap().1.parse::<i64>().unwrap();

        let b_replaced = button_b_str.replace("Button B: ", "");
        let (b_dx_str, b_dy_str) = b_replaced.split_once(", ").unwrap();

        let button_b_dx = b_dx_str.split_once("+").unwrap().1.parse::<i64>().unwrap();

        let button_b_dy = b_dy_str.split_once("+").unwrap().1.parse::<i64>().unwrap();

        let p_replaced = prize_str.replace("Prize: ", "");
        let (px_str, py_str) = p_replaced.split_once(", ").unwrap();

        Ok(ClawMachine {
            x1: button_a_dx,
            y1: button_a_dy,
            x2: button_b_dx,
            y2: button_b_dy,
            px: px_str.split_once("=").unwrap().1.parse::<i64>().unwrap(),
            py: py_str.split_once("=").unwrap().1.parse::<i64>().unwrap(),
        })
    }
}

impl ClawMachine {
    // https://en.wikipedia.org/wiki/Cramer%27s_rule
    fn solve(&self) -> Option<i64> {
        let det = self.x1 * self.y2 - self.x2 * self.y1;
        if det == 0 {
            return None;
        }

        let det_a = self.px * self.y2 - self.x2 * self.py;
        let det_b = self.x1 * self.py - self.px * self.y1;

        if det_a % det != 0 || det_b % det != 0 {
            return None;
        }

        let a = det_a / det;
        let b = det_b / det;

        Some(3 * a + b)
    }
}

fn part_one(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|claw_str| claw_str.parse::<ClawMachine>().unwrap())
        .filter_map(|m| m.solve())
        .sum()
}

fn part_two(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|claw_str| claw_str.parse::<ClawMachine>().unwrap())
        .filter_map(|m| {
            ClawMachine {
                x1: m.x1,
                y1: m.y1,
                x2: m.x2,
                y2: m.y2,
                px: m.px + 10000000000000,
                py: m.py + 10000000000000,
            }
            .solve()
        })
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/day13.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r"Button A: X+94, Y+34
                      Button B: X+22, Y+67
                      Prize: X=8400, Y=5400

                      Button A: X+26, Y+66
                      Button B: X+67, Y+21
                      Prize: X=12748, Y=12176

                      Button A: X+17, Y+86
                      Button B: X+84, Y+37
                      Prize: X=7870, Y=6450

                      Button A: X+69, Y+23
                      Button B: X+27, Y+71
                      Prize: X=18641, Y=10279";
        let res = part_one(input);
        assert_eq!(res, 480)
    }

    #[test]
    fn test_part_two() {
        let input = r"Button A: X+94, Y+34
                      Button B: X+22, Y+67
                      Prize: X=8400, Y=5400

                      Button A: X+26, Y+66
                      Button B: X+67, Y+21
                      Prize: X=12748, Y=12176

                      Button A: X+17, Y+86
                      Button B: X+84, Y+37
                      Prize: X=7870, Y=6450

                      Button A: X+69, Y+23
                      Button B: X+27, Y+71
                      Prize: X=18641, Y=10279";
        let res = part_two(input);
        assert_eq!(res, 875318608908)
    }
}
