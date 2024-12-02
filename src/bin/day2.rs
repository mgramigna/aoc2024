fn is_safe(report: &[usize]) -> bool {
    let pairs: Vec<_> = report.windows(2).collect();
    let strictly_decreasing = pairs.iter().all(|p| p[0] > p[1]);
    let strictly_increasing = pairs.iter().all(|p| p[0] < p[1]);
    let safe_difference = pairs
        .iter()
        .all(|p| p[0].abs_diff(p[1]) >= 1 && p[0].abs_diff(p[1]) <= 3);

    safe_difference && (strictly_increasing || strictly_decreasing)
}

fn is_safe_2(report: &[usize]) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let subset: Vec<_> = report
            .iter()
            .enumerate()
            .filter_map(|(n, e)| if n != i { Some(*e) } else { None })
            .collect();

        if is_safe(&subset) {
            return true;
        }
    }

    false
}

fn part_one(data: &[Vec<usize>]) -> usize {
    data.iter().filter(|report| is_safe(report)).count()
}

fn part_two(data: &[Vec<usize>]) -> usize {
    data.iter().filter(|report| is_safe_2(report)).count()
}

fn main() {
    let lines: Vec<&str> = include_str!("../../inputs/day2.txt").lines().collect();

    let data: Vec<Vec<usize>> = lines
        .iter()
        .map(|line| {
            line.split(' ')
                .map(|ch| ch.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    println!("Part 1: {}", part_one(&data));
    println!("Part 2: {}", part_two(&data));
}
