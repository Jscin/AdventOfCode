use std::fs;

fn is_valid(x: isize, y: isize, rows: isize, cols: isize) -> bool {
    x >= 0 && x < rows && y >= 0 && y < cols
}

fn find_word_in_direction(
    grid: &[Vec<char>],
    word: &[char],
    x: isize,
    y: isize,
    dir_x: isize,
    dir_y: isize,
    rows: isize,
    cols: isize,
) -> bool {
    let word_len = word.len() as isize;

    for i in 0..word_len {
        let nx = x + i * dir_x;
        let ny = y + i * dir_y;

        if !is_valid(nx, ny, rows, cols) || grid[nx as usize][ny as usize] != word[i as usize] {
            return false;
        }
    }

    true
}

fn search_word(grid: &Vec<Vec<char>>, word: &str) -> u64 {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
        (-1, -1), // Up-Left
    ];

    let word_chars: Vec<char> = word.chars().collect();
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i as usize][j as usize] == word_chars[0] {
                for &(dir_x, dir_y) in &directions {
                    if find_word_in_direction(grid, &word_chars, i, j, dir_x, dir_y, rows, cols) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn check_x_mas(grid: &[Vec<char>], x: usize, y: usize, rows: isize, cols: isize) -> bool {
    let x = x as isize;
    let y = y as isize;

    let check_mas = |start_x: isize, start_y: isize, dir_x: isize, dir_y: isize| -> bool {
        let positions = [
            (start_x, start_y),
            (start_x + dir_x, start_y + dir_y),
            (start_x + 2 * dir_x, start_y + 2 * dir_y),
        ];
        let mut chars = vec![];

        for &(px, py) in &positions {
            if is_valid(px, py, rows, cols) {
                chars.push(grid[px as usize][py as usize]);
            } else {
                return false;
            }
        }
        chars == ['M', 'A', 'S'] || chars == ['S', 'A', 'M']
    };

    // Check both diagonals
    check_mas(x - 1, y - 1, 1, 1) && check_mas(x - 1, y + 1, 1, -1)
}

fn search_x_mas(grid: &[Vec<char>]) -> u64 {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i as usize][j as usize] == 'A'
                && check_x_mas(grid, i as usize, j as usize, rows, cols)
            {
                count += 1;
            }
        }
    }

    count
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_one(input: &str) -> u64 {
    let grid = parse_input(input);
    search_word(&grid, "XMAS")
}

fn part_two(input: &str) -> u64 {
    let grid = parse_input(input);
    search_x_mas(&grid)
}

pub fn run() {
    let input = fs::read_to_string("input/day4.txt").expect("Failed to read input file");
    let ans = part_one(&input);
    println!("Part 1: {}", ans);
    let ans = part_two(&input);
    println!("Part 2: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 18);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 9);
    }
}
