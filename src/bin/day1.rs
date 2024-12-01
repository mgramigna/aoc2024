use std::collections::HashMap;

fn part_one(mut left_list: Vec<usize>, mut right_list: Vec<usize>) -> usize {
    right_list.sort_unstable();
    left_list.sort_unstable();

    let mut sum: usize = 0;
    for i in 0..right_list.len() {
        sum += right_list[i].abs_diff(left_list[i]);
    }

    sum
}

fn part_two(left_list: Vec<usize>, right_list: Vec<usize>) -> usize {
    let mut left_list_counts: HashMap<usize, usize> = HashMap::new();
    for rn in right_list {
        let count = left_list_counts.entry(rn).or_insert(0);
        *count += 1;
    }

    let mut sum: usize = 0;
    for ln in left_list {
        let count = left_list_counts.get(&ln).unwrap_or(&0);

        sum += ln * count;
    }
    sum
}

fn main() {
    let lines: Vec<&str> = include_str!("../../inputs/day1.txt").lines().collect();
    let mut left_list: Vec<usize> = vec![];
    let mut right_list: Vec<usize> = vec![];

    for line in lines {
        let (lch, rch) = line.split_once("   ").unwrap();
        left_list.push(lch.parse::<usize>().unwrap());
        right_list.push(rch.parse::<usize>().unwrap());
    }

    assert_eq!(right_list.len(), left_list.len());

    println!(
        "Part 1: {}",
        part_one(left_list.clone(), right_list.clone())
    );

    println!(
        "Part 2: {}",
        part_two(left_list.clone(), right_list.clone())
    );
}
