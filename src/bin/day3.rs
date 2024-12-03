use regex::{Captures, Regex};

fn sum_capture(capture: Captures) -> usize {
    capture[1].parse::<usize>().unwrap() * capture[2].parse::<usize>().unwrap()
}

fn part_one(lines: Vec<&str>) -> usize {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    lines
        .iter()
        .map(|line| re.captures_iter(line).map(sum_capture).sum::<usize>())
        .sum()
}

fn part_two(lines: Vec<&str>) -> usize {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do\(\))|(don't\(\))").unwrap();

    let mut do_mul = true;
    let mut total: usize = 0;

    for line in lines {
        for capture in re.captures_iter(line) {
            match capture.get(0).unwrap().as_str() {
                "don't()" => {
                    do_mul = false;
                }
                "do()" => do_mul = true,
                _ => {
                    if do_mul {
                        total += sum_capture(capture)
                    }
                }
            };
        }
    }

    total
}

fn main() {
    let lines: Vec<&str> = include_str!("../../inputs/day3.txt").lines().collect();

    println!("Part 1: {}", part_one(lines.clone()));
    println!("Part 2: {}", part_two(lines.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let res = part_one(vec![
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        ]);

        assert_eq!(res, 161)
    }

    #[test]
    fn test_part_two() {
        let res = part_two(vec![
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        ]);

        assert_eq!(res, 48)
    }
}
