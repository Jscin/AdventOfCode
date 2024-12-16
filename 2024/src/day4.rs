use std::fs;

fn parser(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_one(input: &str) -> u64 {
    let word_search = parser(input);
}

pub fn run() {
    let input = fs::read_to_string("input/day4.txt").expect("Failed to read input");
    let ans = part_one(&input);
    println!("Part 1: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 18);
    }
}
