use std::collections::HashMap;

fn split_number(number: usize) -> (usize, usize) {
    let digit_count = number.ilog10() + 1;

    let mut left = number;
    let mut right = 0;
    let mut multiplier = 1;

    for _ in 0..digit_count / 2 {
        right += (left % 10) * multiplier;
        multiplier *= 10;
        left /= 10;
    }

    (left, right)
}

fn blink(old_stone_map: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stone_map = HashMap::<usize, usize>::new();

    for (&stone, &count) in old_stone_map {
        match stone {
            0 => *new_stone_map.entry(1).or_default() += count,
            s if (s.ilog10() + 1) % 2 == 0 => {
                let (left, right) = split_number(s);
                *new_stone_map.entry(left).or_default() += count;
                *new_stone_map.entry(right).or_default() += count;
            }
            _ => *new_stone_map.entry(stone * 2024).or_default() += count,
        }
    }

    new_stone_map
}

fn part_one(input: &str) -> usize {
    let blink_count = 25;
    let mut stones = input
        .trim()
        .split(' ')
        .map(|s| (s.parse::<usize>().unwrap(), 1_usize))
        .collect::<HashMap<_, _>>();

    for _ in 0..blink_count {
        stones = blink(&stones);
    }

    stones.values().sum()
}

fn part_two(input: &str) -> usize {
    let blink_count = 75;
    let mut stones = input
        .trim()
        .split(' ')
        .map(|s| (s.parse::<usize>().unwrap(), 1_usize))
        .collect::<HashMap<_, _>>();

    for _ in 0..blink_count {
        stones = blink(&stones);
    }

    stones.values().sum()
}

fn main() {
    let input = include_str!("../../inputs/day11.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_d11() {
        let input = r"125 17";
        let res = part_one(input);

        assert_eq!(res, 55312);
    }

    #[test]
    fn test_part_two_d11() {
        let input = r"125 17";
        let res = part_two(input);

        assert_eq!(res, 55312);
    }
}
