use std::{collections::HashMap, fs};

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

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let split_contents = contents.split_once("\n").unwrap();
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
                split2.0.trim_start_matches("(").to_string(),
                split2.1.trim_end_matches(")").to_string(),
            );
        })
        .collect::<Vec<Node>>();

    for node in node_network.iter() {
        map.insert(&node.name, (&node.left, &node.right));
    }

    let mut current_node = String::from("AAA");

    let mut count = 0;

    for instruction in instructions.iter().cycle() {
        if current_node == "ZZZ" {
            println!("Part 1: {}", count);
            break;
        }
        count += 1;
        let (left, right) = map.get(current_node.as_str()).unwrap();
        match instruction {
            Instruction::Right => current_node = right.to_string(),
            Instruction::Left => current_node = left.to_string(),
        }
    }
}
