use std::thread;
use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
enum Instruction {
    Right,
    Left,
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn new(name: String, left: String, right: String) -> Self {
        Self { name, left, right }
    }
}

fn part_one(
    current_node: String,
    map: HashMap<String, (String, String)>,
    instructions: Vec<Instruction>,
) -> u32 {
    let mut count = 0;
    let mut current_node = current_node;

    for instruction in instructions.iter().cycle() {
        if current_node.ends_with('Z') {
            return count;
        }
        count += 1;
        let (left, right) = map.get(current_node.as_str()).unwrap();
        match instruction {
            Instruction::Right => current_node = right.to_string(),
            Instruction::Left => current_node = left.to_string(),
        }
    }
    count
}

fn part_two(map: HashMap<String, (String, String)>, instructions: Vec<Instruction>) -> u32 {
    // Start at all the values that end in A, and end at all the values that end in Z.
    let starting_nodes: Vec<String> = map
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| x.to_string())
        .collect();
    // Thread this using the part_one function.
    let mut threads = vec![];
    for node in starting_nodes {
        let map_clone = map.clone();
        let instructions_clone = instructions.clone();
        threads.push(thread::spawn(move || {
            part_one(node, map_clone, instructions_clone)
        }));
    }
    // Sum the results.
    let mut sum = 0;
    for thread in threads {
        sum += thread.join().unwrap();
    }

    sum
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let split_contents = contents.split_once('\n').unwrap();
    let instructions: Vec<Instruction> = split_contents
        .0
        .chars()
        .map(|x| match x {
            'R' => Instruction::Right,
            'L' => Instruction::Left,
            _ => panic!("Invalid instruction"),
        })
        .collect();
    let node_network = split_contents
        .1
        .split_terminator('\n')
        .collect::<Vec<&str>>();

    let node_network = node_network
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let split = x.split_once(" = ").unwrap();
            let split2 = split.1.split_once(", ").unwrap();
            return Node::new(
                split.0.to_string(),
                split2.0.trim_start_matches('(').to_string(),
                split2.1.trim_end_matches(')').to_string(),
            );
        })
        .collect::<Vec<Node>>();

    for node in node_network.iter() {
        map.insert(
            node.name.to_string(),
            (node.left.to_string(), node.right.to_string()),
        );
    }

    let res1 = part_one("AAA".to_string(), map.clone(), instructions.clone());
    let res2 = part_two(map, instructions);
    println!("Part one: {}", res1);
    println!("Part two: {}", res2);
}
