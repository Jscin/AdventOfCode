use std::{collections::HashSet, fs};

fn part_one(input: Vec<&str>) -> u32 {
    let mut total_points = 0;
    for card in input {
        if card.is_empty() {
            continue;
        }
        // Split off the card : to get the two parts
        let card: Vec<&str> = card.split(":").collect();
        // Throw away the first part
        let card = card[1];
        // Split the second part into the two parts
        let card: Vec<&str> = card.split("|").collect();

        let win_nums = card[0].trim();
        let our_nums = card[1].trim();

        // Turn these into sets and take the intersection
        let win_nums: Vec<&str> = win_nums.split(" ").collect();
        let our_nums: Vec<&str> = our_nums.split(" ").collect();

        let win_nums: HashSet<u32> = win_nums
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap_or(0))
            .collect();
        let our_nums: HashSet<u32> = our_nums
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap_or(0))
            .collect();

        let intersection: HashSet<u32> = win_nums
            .intersection(&our_nums)
            .cloned()
            .filter(|x| *x != 0)
            .collect();

        let num_win_nums = intersection.len();

        let points = if num_win_nums == 0 {
            0
        } else {
            // 2^(n-1) points where n is the number of win numbers
            2u32.pow(num_win_nums as u32 - 1)
        };
        total_points += points;
    }
    total_points
}

pub fn run() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.split("\n").collect();

    let total_points = part_one(input);

    println!("Total points: {}", total_points);
}
