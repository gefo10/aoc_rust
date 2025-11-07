use regex::Regex;
use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read file");
    let regex = Regex::new(r"(do(n't)?|mul\((-?\d+),\s*(-?\d+)\))").unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for cap in regex.captures_iter(&content) {
        match &cap[0] {
            "do" => {
                enabled = true;
                println!("Enabled multiplication");
            }
            "don't" => {
                enabled = false;
                println!("Disabled multiplication");
            }
            _ => {
                if !enabled {
                    continue;
                }
                let x: i32 = cap[3].parse().unwrap();
                let y: i32 = cap[4].parse().unwrap();
                sum += x * y;
            }
        }
    }

    println!("The total sum is: {}", sum);
}
