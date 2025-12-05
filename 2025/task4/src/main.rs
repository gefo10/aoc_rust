use std::fs;

fn accessable(grid: &Vec<Vec<char>>, r: isize, c: isize) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut counter = 0;

    let dirs = [
        [-1, -1], // top-left
        [0, -1],  // left
        [1, -1],  //bot-left
        [1, 0],   //bot
        [1, 1],   //bot-right
        [0, 1],   //right
        [-1, 1],  //top-right
        [-1, 0],  //top
    ];

    //println!("Checking now paper {} {}", r, c);
    for dir in &dirs {
        let r_new = r + dir[0];
        let c_new = c + dir[1];

        if r_new < 0 || c_new < 0 || r_new >= rows || c_new >= cols {
            continue;
        }

        if grid[r_new as usize][c_new as usize] == '@' {
            //println!("Found adjacent paper at {} {}", r_new, c_new);
            counter += 1;
        }
    }

    counter < 4
}

fn solve(content: &str) -> i32 {
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    for row_index in 0..rows {
        for col_index in 0..cols {
            if grid[row_index][col_index] == '@'
                && accessable(&grid, row_index as isize, col_index as isize)
            {
                count += 1;
            }
        }
    }

    count
}

fn solve_v2(content: &str) -> i32 {
    let mut grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    loop {
        let mut edited = false;
        for row_index in 0..rows {
            for col_index in 0..cols {
                if grid[row_index][col_index] == '@'
                    && accessable(&grid, row_index as isize, col_index as isize)
                {
                    count += 1;
                    grid[row_index][col_index] = 'x';
                    edited = true;
                }
            }
        }

        println!("Removed {} rolls of paper.", count);
        if edited == false {
            break;
        }
    }

    count
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to parse");
    let content_test = fs::read_to_string("test.txt").expect("Failed to parse");

    let content_result = solve(&content);
    let content_test_result = solve(&content_test);

    println!(
        "Total accessable rolls of paper (test): {}",
        content_test_result
    );
    assert!(content_test_result == 13);
    println!("Total accessable rolls of paper: {}", content_result);

    println!("PART 2 ------------");

    let content_result = solve_v2(&content);
    let content_test_result = solve_v2(&content_test);

    println!(
        "Total accessable rolls of paper (test): {}",
        content_test_result
    );
    assert!(content_test_result == 43);
    println!("Total accessable rolls of paper: {}", content_result);
}
