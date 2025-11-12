use std::fs;

struct Position {
    x: usize,
    y: usize,
}

pub type Direction = (isize, isize);

static WORD: &str = "MAS";

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read file");

    let dirs: [Direction; 8] = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];

    let chars = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let rows = chars.len();
    let cols = chars[0].len();
    let mut count = 0;
    let mut positions: Vec<(Position, Direction)> = Vec::new();

    let word_chars: Vec<char> = WORD.chars().collect();
    let word_chars_rev: Vec<char> = word_chars.iter().cloned().rev().collect();

    // Look for 'A' as the center of the X
    for i in 1..rows - 1 {
        // Skip edges since we need neighbors
        for j in 1..cols - 1 {
            if chars[i][j] == 'A' {
                if is_xmas_pattern(&chars, i, j) {
                    count += 1;
                }
            }
        }
    }

    // for i in 0..rows {
    //     for j in 0..cols {
    //         if chars[i][j] == word_chars[0] || chars[i][j] == word_chars_rev[0] {
    //             for &dir in &dirs {
    //                 if check_word(&chars, &word_chars, i, j, dir) {
    //                     count += 1;
    //                     positions.push((Position { x: j, y: i }, dir));
    //                 }
    //             }
    //         }
    //     }
    // }
    //
    println!("Total occurrences of '{}': {}", WORD, count);
}

fn check_word(
    chars: &Vec<Vec<char>>,
    word_chars: &Vec<char>,
    start_i: usize,
    start_j: usize,
    dir: Direction,
) -> bool {
    let rows = chars.len() as isize;
    let cols = chars[0].len() as isize;
    let word_len = word_chars.len() as isize;

    for k in 0..word_len {
        let new_i = start_i as isize + k * dir.0;
        let new_j = start_j as isize + k * dir.1;

        if new_i < 0 || new_i >= rows || new_j < 0 || new_j >= cols {
            return false;
        }

        if chars[new_i as usize][new_j as usize] != word_chars[k as usize] {
            return false;
        }
    }

    true
}

fn is_xmas_pattern(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    // Check both diagonals
    // Diagonal 1: top-left to bottom-right
    let diag1_tl = chars[i - 1][j - 1]; // top-left
    let diag1_br = chars[i + 1][j + 1]; // bottom-right

    // Diagonal 2: top-right to bottom-left
    let diag2_tr = chars[i - 1][j + 1]; // top-right
    let diag2_bl = chars[i + 1][j - 1]; // bottom-left

    // Check if diagonal 1 is "MAS" or "SAM"
    let diag1_valid = (diag1_tl == 'M' && diag1_br == 'S') || (diag1_tl == 'S' && diag1_br == 'M');

    // Check if diagonal 2 is "MAS" or "SAM"
    let diag2_valid = (diag2_tr == 'M' && diag2_bl == 'S') || (diag2_tr == 'S' && diag2_bl == 'M');

    // Both diagonals must be valid
    diag1_valid && diag2_valid
}
