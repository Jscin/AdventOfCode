use std::fs;

fn part_one(input: Vec<&str>) -> u32 {
    // Break the input into a 2d character vector
    let schematic = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Pad the schematic with a column of dots on each side for sanity
    let padded_schematic = schematic
        .iter()
        .map(|row| {
            let mut padded_row = row.clone();
            padded_row.insert(0, '.');
            padded_row.push('.');
            padded_row
        })
        .collect::<Vec<Vec<char>>>();

    fn is_dot(char: char) -> bool {
        char.eq_ignore_ascii_case(&'.')
    }

    fn is_digit(char: char) -> bool {
        char.is_ascii_digit()
    }

    fn is_valid_part(schematic: Vec<Vec<char>>, x: usize, y: usize) -> bool {
        fn top_row_check(schematic: Vec<Vec<char>>, x: usize, y: usize) -> bool {
            // Check adjacent columns for neither being a digit nor a dot
            if (!is_digit(schematic[y][x - 1]) && !is_digit(schematic[y][x + 1]))
                && (!is_dot(schematic[y][x - 1]) && !is_dot(schematic[y][x + 1]))
            {
                return true;
            // check below and diagonals for not being a digit or dot
            } else if (!is_digit(schematic[y + 1][x - 1])
                && !is_digit(schematic[y + 1][x])
                && !is_digit(schematic[y + 1][x + 1]))
                && (!is_dot(schematic[y + 1][x - 1])
                    && !is_dot(schematic[y + 1][x])
                    && !is_dot(schematic[y + 1][x + 1]))
            {
                return true;
            }
            false
        }

        fn bottom_row_check(schematic: Vec<Vec<char>>, x: usize, y: usize) -> bool {
            // Check adjacent columns for neither being a digit nor a dot
            if (!is_digit(schematic[y][x - 1]) && !is_digit(schematic[y][x + 1]))
                && (!is_dot(schematic[y][x - 1]) && !is_dot(schematic[y][x + 1]))
            {
                return true;
            // check above and diagonals for not being a digit or dot
            } else if (!is_digit(schematic[y - 1][x - 1])
                && !is_digit(schematic[y - 1][x])
                && !is_digit(schematic[y - 1][x + 1]))
                || (!is_dot(schematic[y - 1][x - 1])
                    && !is_dot(schematic[y - 1][x])
                    && !is_dot(schematic[y - 1][x + 1]))
            {
                return true;
            }
            false
        }

        let mut valid = false;

        if y == 0 {
            valid = top_row_check(schematic, x, y);
        } else if y == schematic.len() - 1 {
            valid = bottom_row_check(schematic, x, y);
        } else {
            // check all sides for not being a digit or dot
            if (!is_digit(schematic[y - 1][x - 1])
                && !is_digit(schematic[y - 1][x])
                && !is_digit(schematic[y - 1][x + 1])
                && !is_digit(schematic[y][x - 1])
                && !is_digit(schematic[y][x + 1])
                && !is_digit(schematic[y + 1][x - 1])
                && !is_digit(schematic[y + 1][x])
                && !is_digit(schematic[y + 1][x + 1]))
                || (!is_dot(schematic[y - 1][x - 1])
                    && !is_dot(schematic[y - 1][x])
                    && !is_dot(schematic[y - 1][x + 1])
                    && !is_dot(schematic[y][x - 1])
                    && !is_dot(schematic[y][x + 1])
                    && !is_dot(schematic[y + 1][x - 1])
                    && !is_dot(schematic[y + 1][x])
                    && !is_dot(schematic[y + 1][x + 1]))
            {
                valid = true;
            }
        }

        valid
    }

    let mut sum = 0;
    padded_schematic.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, col)| {
            if is_digit(*col) {
                let mut num = String::new();
                if is_valid_part(padded_schematic.clone(), x, y) {
                    // Once the valid part is found, check the row for the rest of the number
                    // Walk left from the current column until a dot is found
                    // then push the leftmost digit to the string and continue right until a dot is found
                    let mut current_x = x;
                    while current_x > 0 && is_digit(padded_schematic[y][current_x - 1]) {
                        current_x -= 1;
                    }
                    while current_x < padded_schematic[y].len()
                        && is_digit(padded_schematic[y][current_x])
                    {
                        num.push(padded_schematic[y][current_x]);
                        current_x += 1;
                    }
                    println!("Part Num Found: {num}");
                } else {
                    println!("Part Num Not Found: {num}");
                }
                sum += num.parse::<u32>().unwrap_or(0);
            };
        });
    });
    sum
}

fn main() {
    let input = fs::read_to_string("ex.txt").unwrap();
    println!(
        "Part one: {}",
        part_one(input.lines().collect::<Vec<&str>>())
    );
}
