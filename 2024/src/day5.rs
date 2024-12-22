use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn parse_input(input: &str) -> (HashMap<u64, Vec<u64>>, Vec<Vec<u64>>) {
    let mut parts = input.split("\n\n");
    let ordering_rules = parts.next().unwrap();
    let pages = parts.next().unwrap();

    let mut rules = HashMap::new();
    for line in ordering_rules.lines() {
        let mut parts = line.split('|');
        let first = parts.next().unwrap().parse().unwrap();
        let second = parts.next().unwrap().parse().unwrap();
        rules.entry(first).or_insert_with(Vec::new).push(second);
    }

    let pages = pages
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();
    (rules, pages)
}

fn is_valid_order(ordering_rules: &HashMap<u64, Vec<u64>>, pages: &[u64]) -> bool {
    let page_positions: HashMap<u64, usize> = pages
        .iter()
        .enumerate()
        .map(|(idx, &page)| (page, idx))
        .collect();

    for (&start, ends) in ordering_rules {
        if let Some(&start_pos) = page_positions.get(&start) {
            for &end in ends {
                if let Some(&end_pos) = page_positions.get(&end) {
                    if start_pos >= end_pos {
                        return false; // Rule violated
                    }
                }
            }
        }
    }
    true
}

fn part_one(ordering_rules: &HashMap<u64, Vec<u64>>, pages: &[Vec<u64>]) -> u64 {
    pages
        .iter()
        .filter(|update| is_valid_order(ordering_rules, update))
        .map(|valid_update| valid_update[valid_update.len() / 2]) // Get middle element
        .sum()
}

fn topological_sort(ordering_rules: &HashMap<u64, Vec<u64>>, pages: &[u64]) -> Vec<u64> {
    let mut in_degree = HashMap::new();
    let mut adjacency_list: HashMap<u64, Vec<u64>> = HashMap::new();
    let pages_set: HashSet<u64> = pages.iter().copied().collect();

    for &page in pages {
        in_degree.insert(page, 0);
        adjacency_list.insert(page, Vec::new());
    }

    for (&start, ends) in ordering_rules {
        if pages_set.contains(&start) {
            for &end in ends {
                if pages_set.contains(&end) {
                    adjacency_list.entry(start).or_default().push(end);
                    *in_degree.entry(end).or_default() += 1;
                }
            }
        }
    }

    let mut queue = VecDeque::new();
    for (&page, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(page);
        }
    }

    let mut sorted_pages = Vec::new();
    while let Some(current) = queue.pop_front() {
        sorted_pages.push(current);
        if let Some(neighbors) = adjacency_list.get(&current) {
            for &neighbor in neighbors {
                let degree = in_degree.get_mut(&neighbor).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    sorted_pages
}

fn part_two(ordering_rules: &HashMap<u64, Vec<u64>>, pages: &[Vec<u64>]) -> u64 {
    pages
        .iter()
        .filter(|update| !is_valid_order(ordering_rules, update)) // Find incorrect updates
        .map(|incorrect_update| {
            let corrected_order = topological_sort(ordering_rules, incorrect_update);
            corrected_order[corrected_order.len() / 2] // Get middle element
        })
        .sum()
}

pub fn run() {
    let input = fs::read_to_string("input/day5.txt").unwrap();
    let (ordering_rules, pages) = parse_input(&input);
    let ans = part_one(&ordering_rules, &pages);
    println!("Part 1: {}", ans);
    let ans = part_two(&ordering_rules, &pages);
    println!("Part 2: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;
    const ORDERING_RULES: &str = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n";
    const PAGES: &str =
        "75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47\n";

    #[test]
    fn test_part_one() {
        let input = format!("{}\n{}", ORDERING_RULES, PAGES);
        let (ordering_rules, pages) = parse_input(&input);
        assert_eq!(part_one(&ordering_rules, &pages), 143);
    }

    #[test]
    fn test_part_two() {
        let input = format!("{}\n{}", ORDERING_RULES, PAGES);
        let (ordering_rules, pages) = parse_input(&input);
        assert_eq!(part_two(&ordering_rules, &pages), 123);
    }
}
