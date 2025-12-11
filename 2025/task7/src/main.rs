use std::collections::VecDeque;
use std::fs;

use std::collections::HashSet;

use std::collections::HashMap;

fn solve_v2_optimized(content: &str) -> usize {
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    // Find Start
    let (start_col, _) = grid[0]
        .iter()
        .enumerate()
        .find(|(_, c)| **c == 'S')
        .expect("No Start Found");

    let mut memo = HashMap::new();

    // Start the recursive count from (0, start_col)
    count_timelines(0, start_col, &grid, &mut memo)
}

fn count_timelines(
    row: usize,
    col: usize,
    grid: &Vec<Vec<char>>,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // 1. Base Case: We reached the bottom successfully
    if row == grid.len() {
        return 1;
    }

    // 2. Check Cache
    if let Some(&count) = memo.get(&(row, col)) {
        return count;
    }

    // 3. Calculate based on current cell type
    let count = match grid[row][col] {
        '^' => {
            // Splitter logic:
            // Beam splits to (row, col-1) and (row, col+1).
            // However, the problem implies the split happens and the *next* step
            // is effectively moving down from those new positions.
            // So we look at (row + 1, col - 1) and (row + 1, col + 1).

            let mut total = 0;

            // Left branch (check bounds)
            if col > 0 {
                total += count_timelines(row + 1, col - 1, grid, memo);
            }

            // Right branch (check bounds)
            if col < grid[row].len() - 1 {
                total += count_timelines(row + 1, col + 1, grid, memo);
            }

            total
        }
        _ => {
            // Empty space or Start 'S': continue straight down
            count_timelines(row + 1, col, grid, memo)
        }
    };

    // 4. Save to Cache and return
    memo.insert((row, col), count);
    count
}
fn solve_v2(content: &str) -> usize {
    //println!("Content is : \n{}\n\n", content);
    let grid: Vec<Vec<char>> = content
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut col_indexes = Vec::new();
    let mut timelines = HashSet::new();
    let mut path = Vec::new();

    //we need to find S to start
    let (start_index, _) = grid[0]
        .iter()
        .enumerate()
        .find(|(_, col)| **col == 'S')
        .unwrap();

    path.push((0usize, start_index));
    //println!("Found S at: {}", start_index);
    col_indexes.push((0usize, start_index, path));

    while !col_indexes.is_empty() {
        //println!("{:?}", col_indexes);
        let (row, col, mut path) = col_indexes.pop().unwrap();
        //println!("Current indices are: {:?}", current_indices);

        let next_row = row + 1;
        if next_row == grid.len() {
            println!("Path found!\n {:?}", path);
            timelines.insert(path);
            continue;
        }
        if grid[next_row][col] == '^' {
            if col > 0 {
                let mut left_path = path.clone();
                left_path.push((next_row, col - 1));
                col_indexes.push((next_row, col - 1, left_path));
            }

            if col < grid[next_row].len() - 1 {
                let mut right_path = path.clone();
                right_path.push((next_row, col + 1));
                col_indexes.push((next_row, col + 1, right_path));
            }
        } else {
            path.push((next_row, col));
            col_indexes.push((next_row, col, path));
        }
    }

    timelines.len()
}
fn solve(content: &str) -> u64 {
    //println!("Content is : \n{}\n\n", content);
    let grid: Vec<Vec<char>> = content
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let mut col_indexes = VecDeque::new();

    let mut result = 0;

    for row in 0..rows {
        if row == 0 && col_indexes.is_empty() {
            //we need to find S to start
            let (start_index, _) = grid[row]
                .iter()
                .enumerate()
                .find(|(_, col)| **col == 'S')
                .unwrap();

            //println!("Found S at: {}", start_index);
            col_indexes.push_back(vec![start_index]);
            continue;
        } else if col_indexes.is_empty() {
            println!("Ending early...");
            return result;
        }

        let current_indices = col_indexes.pop_front().unwrap();
        //println!("Current indices are: {:?}", current_indices);

        let mut next_indices = vec![];
        for i in current_indices {
            if grid[row][i] == '^' {
                if i > 0 && !next_indices.contains(&(i - 1)) {
                    next_indices.push(i - 1);
                }

                if i < grid[row].len() - 1 && !next_indices.contains(&(i + 1)) {
                    next_indices.push(i + 1);
                }

                result += 1;
            } else {
                if !next_indices.contains(&i) {
                    next_indices.push(i);
                }
            }
        }

        col_indexes.push_back(next_indices);
    }

    result
}
fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let content_test = fs::read_to_string("test.txt").unwrap();

    let result = solve(&content);
    let result_test = solve(&content_test);

    println!("Grand total (test): {}", result_test);
    assert!(result_test == 21);

    println!("Grand total: {}", result);

    println!("PART 2 -------------------");

    let result_p2 = solve_v2_optimized(&content);
    let result_test_p2 = solve_v2_optimized(&content_test);

    println!("Grand total (test): {}", result_test_p2);
    assert!(result_test_p2 == 40);

    println!("Grand total: {}", result_p2);
    //println!("PART 3 -------------------");

    //let result_p2 = solve_v3(&content);
    //let result_test_p2 = solve_v3(&content_test);

    //println!("Grand total (test): {}", result_test_p2);
    //assert!(result_test_p2 == 40);

    //println!("Grand total: {}", result_p2);
}
