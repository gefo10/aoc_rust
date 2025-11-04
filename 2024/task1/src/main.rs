use std::fs;

fn main() {
    let file_path = "input.txt";

    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let mut left: Vec<&str> = Vec::<&str>::new();
    let mut right: Vec<&str> = Vec::<&str>::new();

    for line in content.split("\n") {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts.as_slice() {
            [l, r] => {
                left.push(l);
                right.push(r);
            }
            _ => {
                println!("Error: Expected exactly 2 parts");
            }
        }
    }

    left.sort_by(|a, b| b.cmp(a));
    right.sort_by(|a, b| b.cmp(a));

    let mut sum = 0;
    while !left.is_empty() && !right.is_empty() {
        let l = match left.pop().expect("failed").parse::<i32>() {
            Ok(number) => number,
            Err(_) => {
                panic!("FAILED to parse str");
            }
        };
        let r = match right.pop().expect("failed").parse::<i32>() {
            Ok(number) => number,
            Err(_) => {
                panic!("FAILED to parse str");
            }
        };

        sum += (r - l).abs();
    }

    println!("Success ✅: answer is {}", sum);
}
