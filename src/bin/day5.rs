use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
struct PageData {
    ordering: HashMap<usize, HashSet<usize>>,
    updates: Vec<Vec<usize>>,
}

impl FromStr for PageData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ordering_str, updates_str) = s.split_once("\n\n").unwrap();
        let mut ordering = HashMap::<usize, HashSet<usize>>::new();

        for line in ordering_str.lines() {
            let (xs, ys) = line.trim().split_once('|').unwrap();
            let (x, y) = (xs.parse::<usize>().unwrap(), ys.parse::<usize>().unwrap());

            ordering
                .entry(x)
                .and_modify(|s| {
                    s.insert(y);
                })
                .or_insert(HashSet::from([y]));
        }

        let updates = updates_str
            .lines()
            .map(|l| {
                l.trim()
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect()
            })
            .collect::<Vec<_>>();

        Ok(PageData { ordering, updates })
    }
}

fn part_one(data: &PageData) -> usize {
    let mut total: usize = 0;

    for update in &data.updates {
        if update.is_sorted_by(|a, b| data.ordering.get(a).unwrap_or(&HashSet::new()).contains(b)) {
            total += update[update.len() / 2];
        }
    }

    total
}

fn part_two(data: &PageData) -> usize {
    let mut total: usize = 0;

    for update in &data.updates {
        if !update.is_sorted_by(|a, b| data.ordering.get(a).unwrap_or(&HashSet::new()).contains(b))
        {
            let mut new_update = update.clone();
            new_update.sort_by(|a, b| {
                data.ordering
                    .get(a)
                    .unwrap_or(&HashSet::new())
                    .contains(b)
                    .cmp(&true)
            });

            total += new_update[new_update.len() / 2];
        }
    }

    total
}

fn main() {
    let input = include_str!("../../inputs/day5.txt");
    let page_data = input.parse::<PageData>().unwrap();

    println!("Part 1: {}", part_one(&page_data));
    println!("Part 2: {}", part_two(&page_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_d5() {
        let input = r"47|53
                      97|13
                      97|61
                      97|47
                      75|29
                      61|13
                      75|53
                      29|13
                      97|29
                      53|29
                      61|53
                      97|53
                      61|29
                      47|13
                      75|47
                      97|75
                      47|61
                      75|61
                      47|29
                      75|13
                      53|13

                      75,47,61,53,29
                      97,61,53,29,13
                      75,29,13
                      75,97,47,61,53
                      61,13,29
                      97,13,75,29,47";

        let res = part_one(&input.parse::<PageData>().unwrap());

        assert_eq!(res, 143)
    }

    #[test]
    fn test_part_two_d5() {
        let input = r"47|53
                      97|13
                      97|61
                      97|47
                      75|29
                      61|13
                      75|53
                      29|13
                      97|29
                      53|29
                      61|53
                      97|53
                      61|29
                      47|13
                      75|47
                      97|75
                      47|61
                      75|61
                      47|29
                      75|13
                      53|13

                      75,47,61,53,29
                      97,61,53,29,13
                      75,29,13
                      75,97,47,61,53
                      61,13,29
                      97,13,75,29,47";

        let res = part_two(&input.parse::<PageData>().unwrap());

        assert_eq!(res, 123)
    }
}
