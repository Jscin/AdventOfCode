use std::fs;

enum Color {
    Red,
    Blue,
    Green,
}

fn part_one(input: Vec<&str>) -> u32 {
    let mut id_sum = 0;
    let mut id = 1;
    for line in input {
        let games = line.split_once(":").unwrap();
        let games = games.1.split_whitespace().collect::<Vec<&str>>();
        let mut res: Vec<(u32, Color)> = Vec::new();
        for (i, game) in games.iter().enumerate() {
            let num: u32 = if i % 2 == 0 {
                game.parse::<u32>().unwrap()
            } else {
                continue;
            };

            let game = games[i + 1];
            let color = if game.starts_with("red") {
                Color::Red
            } else if game.starts_with("blue") {
                Color::Blue
            } else if game.starts_with("green") {
                Color::Green
            } else {
                continue;
            };

            res.push((num, color));
            continue;
        }

        let mut red_max = 0;
        let mut blue_max = 0;
        let mut green_max = 0;
        for group in res {
            match group.1 {
                Color::Red => red_max = if red_max < group.0 { group.0 } else { red_max },
                Color::Blue => {
                    blue_max = if blue_max < group.0 {
                        group.0
                    } else {
                        blue_max
                    }
                }
                Color::Green => {
                    green_max = if green_max < group.0 {
                        group.0
                    } else {
                        green_max
                    }
                }
            }
        }
        if red_max <= 12 && green_max <= 13 && blue_max <= 14 {
            id_sum += id;
        }
        id += 1;
    }

    id_sum
}

fn part_two(input: Vec<&str>) -> u32 {
    let mut sum = 0;
    for line in input {
        let games = line.split_once(":").unwrap();
        let games = games.1.split_whitespace().collect::<Vec<&str>>();
        let mut res: Vec<(u32, Color)> = Vec::new();
        for (i, game) in games.iter().enumerate() {
            let num: u32 = if i % 2 == 0 {
                game.parse::<u32>().unwrap()
            } else {
                continue;
            };

            let game = games[i + 1];
            let color = if game.starts_with("red") {
                Color::Red
            } else if game.starts_with("blue") {
                Color::Blue
            } else if game.starts_with("green") {
                Color::Green
            } else {
                continue;
            };

            res.push((num, color));
            continue;
        }

        let mut red_max = 0;
        let mut blue_max = 0;
        let mut green_max = 0;
        for group in res {
            match group.1 {
                Color::Red => red_max = if red_max < group.0 { group.0 } else { red_max },
                Color::Blue => {
                    blue_max = if blue_max < group.0 {
                        group.0
                    } else {
                        blue_max
                    }
                }
                Color::Green => {
                    green_max = if green_max < group.0 {
                        group.0
                    } else {
                        green_max
                    }
                }
            }
        }
        sum += red_max * blue_max * green_max;
    }
    sum
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let input: Vec<&str> = input.lines().collect();

    println!("Part One: {}", part_one(input.clone()));
    println!("Part Two: {}", part_two(input));
}
