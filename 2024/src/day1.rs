use std::{collections::HashMap, fs};

fn parse_list(input: &str) -> (Vec<u64>, Vec<u64>) {
    let location_id = input
        .lines()
        .map(|line| {
            let left = line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let right = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u64>()
                .unwrap();

            (left, right)
        })
        .collect::<Vec<(u64, u64)>>();
    let mut left_id = location_id.iter().map(|(l, _)| *l).collect::<Vec<u64>>();
    let mut right_id = location_id.iter().map(|(_, r)| *r).collect::<Vec<u64>>();

    left_id.sort_unstable();
    right_id.sort_unstable();
    (left_id, right_id)
}

fn part_one(input: &str) -> u64 {
    let (left_id, right_id) = parse_list(input);
    left_id
        .iter()
        .zip(right_id)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn part_two(input: &str) -> u64 {
    let (left_id, right_id) = parse_list(input);

    let mut count_map: HashMap<u64, u64> = HashMap::new();
    for value in &right_id {
        *count_map.entry(*value).or_insert(0) += 1;
    }

    let similarity_score: u64 = left_id
        .iter()
        .map(|&value| {
            let count = count_map.get(&value).unwrap_or(&0);
            value * count
        })
        .sum();

    similarity_score
}

pub fn run() {
    let input = fs::read_to_string("input/day1.txt").unwrap();
    let ans = part_one(input.as_str());
    println!("Answer: {:?}", ans);
    let ans = part_two(input.as_str());
    println!("Answer: {:?}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 11);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 31);
    }
}
