use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref NUM_MAP: HashMap<&'static str, i32> = {
        HashMap::from([
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ])
    };
}

fn extract_numbers_part_one(input: Vec<&str>) -> i32 {
    let mut results = Vec::new();
    for str in input {
        let mut result = String::new();
        for c in str.chars() {
            if c.is_ascii_digit() {
                result.push(c);
                break;
            }
        }

        for c in str.chars().rev() {
            if c.is_ascii_digit() {
                result.push(c);
                break;
            }
        }
        results.push(result);
    }

    let mut sum = 0;
    for result in results {
        if result.len() == 2 {
            let num = result.parse::<i32>().unwrap();
            sum += num;
        }
    }
    sum
}

fn extract_numbers_part_two(input: Vec<&str>) -> i32 {
    fn extract_num_forward(line: &str) -> i32 {
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                return c.to_digit(10).unwrap() as i32;
            }

            if c.is_alphabetic() {
                let mut result = String::new();

                for c in line.chars().skip(i) {
                    if c.is_alphabetic() {
                        result.push(c);

                        if result.len() >= 3 {
                            let num = NUM_MAP.get(result.as_str());
                            match num {
                                Some(num) => return *num,
                                None => continue,
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        0
    }

    fn extract_num_reverse(line: &str) -> i32 {
        for (i, c) in line.chars().rev().enumerate() {
            if c.is_ascii_digit() {
                return c.to_digit(10).unwrap() as i32;
            }

            if c.is_alphabetic() {
                let mut result = String::new();

                for c in line.chars().rev().skip(i) {
                    if c.is_alphabetic() {
                        result.push(c);

                        if result.len() >= 3 {
                            let num =
                                NUM_MAP.get(result.chars().rev().collect::<String>().as_str());
                            match num {
                                Some(num) => return *num,
                                None => continue,
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        0
    }

    fn process_line(line: &str) -> i32 {
        let first = extract_num_forward(line);
        let last = extract_num_reverse(line);

        let mut answer = String::new();

        answer.push_str(&first.to_string());
        answer.push_str(&last.to_string());

        answer.parse::<i32>().unwrap()
    }

    let mut sum = 0;
    for line in input {
        sum += process_line(line);
    }
    sum
}

pub fn run() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let input = input.split("\n").collect::<Vec<&str>>();
    let res1 = extract_numbers_part_one(input.clone());
    let res2 = extract_numbers_part_two(input);

    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);
}
