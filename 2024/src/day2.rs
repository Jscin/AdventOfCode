use std::{cmp::Ordering, fs};

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &[u64]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut ascending = true;
    let mut descending = true;

    for window in report.windows(2) {
        let diff = (window[1] as i64 - window[0] as i64).abs();
        if !(1..=3).contains(&diff) {
            return false;
        }
        match window[0].cmp(&window[1]) {
            Ordering::Less => descending = false,
            Ordering::Greater => ascending = false,
            Ordering::Equal => {}
        }
    }

    ascending || descending
}

fn part_one(input: &str) -> u64 {
    let reports = parse_input(input);
    reports.iter().filter(|&report| is_safe(report)).count() as u64
}

fn is_safe_with_dampener(report: &[u64]) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i);
        if is_safe(&modified_report) {
            return true;
        }
    }

    false
}

fn part_two(input: &str) -> u64 {
    let reports = parse_input(input);
    reports
        .iter()
        .filter(|&report| is_safe_with_dampener(report))
        .count() as u64
}

pub fn run() {
    let input = fs::read_to_string("input/day2.txt").unwrap();
    let ans = part_one(input.as_str());
    println!("Part 1: {}", ans);
    let ans = part_two(input.as_str());
    println!("Part 2: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 4);
    }
}
