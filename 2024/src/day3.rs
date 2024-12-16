use regex::Regex;
use std::fs;

fn parser(input: &str) -> Vec<(u64, u64)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
        .collect()
}

fn part_one(input: &str) -> u64 {
    let groups = parser(input);
    groups.iter().map(|(a, b)| a * b).sum()
}

fn part_two(input: &str) -> u64 {
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\(\d+,\d+\))").unwrap();
    let mut enabled = true;
    let mut result = 0;

    for cap in re.captures_iter(input) {
        match &cap[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if enabled => {
                if let Some(caps) = Regex::new(r"mul\((\d+),(\d+)\)").unwrap().captures(&cap[0]) {
                    let a = caps[1].parse::<u64>().unwrap();
                    let b = caps[2].parse::<u64>().unwrap();
                    result += a * b;
                }
            }
            _ => {}
        }
    }

    result
}

pub fn run() {
    let input = fs::read_to_string("input/day3.txt").expect("Failed to read file");
    let ans = part_one(&input);
    println!("Part 1: {}", ans);
    let ans = part_two(&input);
    println!("Part 2: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 161);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT2), 48);
    }
}
