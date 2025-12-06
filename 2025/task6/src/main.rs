use std::fs;
fn solve_v2(content: &str) -> i64 {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    // Find max length and pad
    let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(max_len, ' ');
            chars
        })
        .collect();

    let rows = grid.len();
    let cols = max_len;

    let mut problems = Vec::new();
    let mut current_problem = Vec::new();

    // Read columns from left to right (will reverse later)
    for col in 0..cols {
        let column: Vec<char> = (0..rows).map(|row| grid[row][col]).collect();

        // Check if this column is all spaces (separator)
        let is_separator = column.iter().all(|&ch| ch == ' ');

        if is_separator {
            if !current_problem.is_empty() {
                problems.push(current_problem.clone());
                current_problem.clear();
            }
        } else {
            current_problem.push(column);
        }
    }

    // Don't forget last problem
    if !current_problem.is_empty() {
        problems.push(current_problem);
    }

    // Process problems (read right-to-left)
    let mut grand_total = 0i64;

    for problem in problems.iter().rev() {
        // Last row is the operator
        let operator = problem.first().unwrap().last().unwrap();

        //println!("Operator is: {:?}", problem.first().unwrap());
        // Extract numbers from columns (each column is one number)
        let mut numbers = Vec::new();

        for col in problem.iter() {
            // Build number from top to bottom (excluding operator row)
            let num_str: String = col[..col.len() - 1]
                .iter()
                .filter(|&&ch| ch != ' ')
                .collect();

            if !num_str.is_empty() {
                if let Ok(num) = num_str.parse::<i64>() {
                    numbers.push(num);
                }
            }
        }

        // Calculate result
        let result = match operator {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => 0,
        };

        println!("Problem: {:?} {} = {}", numbers, operator, result);
        grand_total += result;
    }

    grand_total
}
fn solve(content: &str) -> i64 {
    let grid: Vec<Vec<&str>> = content
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            line.split(" ")
                .filter(|l| !l.is_empty())
                .map(|n| n.trim())
                .collect()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let operator_row = rows - 1;

    let mut result = 0;

    for col in 0..cols {
        let operator = grid[operator_row][col];
        let mut temp_result = if operator == "+" { 0i64 } else { 1i64 };
        for row in 0..rows - 1 {
            if operator == "+" {
                temp_result += grid[row][col].parse::<i64>().unwrap();
            } else {
                temp_result *= grid[row][col].parse::<i64>().unwrap();
            }
        }

        result += temp_result;
    }

    result
}
fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let content_test = fs::read_to_string("test.txt").unwrap();

    let result = solve(&content);
    let result_test = solve(&content_test);

    println!("Grand total (test): {}", result_test);
    assert!(result_test == 4277556);

    println!("Grand total: {}", result);

    println!("PART 2 -------------------");

    let result_p2 = solve_v2(&content);
    let result_test_p2 = solve_v2(&content_test);

    println!("Grand total (test): {}", result_test_p2);
    assert!(result_test_p2 == 3263827);

    println!("Grand total: {}", result_p2);
}
