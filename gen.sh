#!/bin/bash

p="src/bin/$1.rs"
i="inputs/$1.txt"


touch "$p"
touch "$i"

cat > $p <<- EOM
fn part_one(input: &str) -> usize {
    todo!()
}

fn part_two(input: &str) -> usize {
    todo!()
}

fn main() {
    let input = include_str!("../../inputs/$1.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r"";

        let res = part_one(input);
    }

    #[test]
    fn test_part_two() {
        let input = r"";

        let res = part_two(input);
    }
}
EOM

nvim "$i"
