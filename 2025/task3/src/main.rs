use std::fs;
fn find_highest_joltage_v2(bank: &str) -> u64 {
    let mut to_keep = vec![];

    let target_len = 12;
    let mut start_index = 0;

    for position in 0..target_len {
        let mut index: usize = 0;
        let mut max = '0';

        let remaining_needed = target_len - position - 1;
        let end_index = bank.len() - remaining_needed;

        for i in start_index..end_index {
            let n = bank.chars().nth(i).unwrap();

            if n > max {
                max = n;
                index = i;
            }
        }

        to_keep.push(index);
        start_index = index + 1;
    }

    bank.chars()
        .enumerate()
        .filter(|(i, _)| to_keep.contains(i))
        .map(|(_, ch)| ch)
        .collect::<String>()
        .parse()
        .expect("Failed to parse String to u64")
}

fn find_highest_joltage(bank: &str) -> i32 {
    let mut max: i32 = -1;
    let mut max_index: usize = 0;

    for (i, number_str) in bank.chars().enumerate() {
        let number: i32 = number_str.to_digit(10).unwrap() as i32;

        if max < number {
            max = number;
            max_index = i;
        }
    }

    let mut max_left_side: i32 = -1;

    for index in 0..max_index {
        let number = match bank.chars().nth(index) {
            Some(value) => value.to_digit(10).unwrap(),
            None => panic!("Not a valid digit"),
        };

        if max_left_side < number as i32 {
            max_left_side = number as i32;
        }
    }

    let mut max_right_side: i32 = -1;
    for index in max_index + 1..bank.len() {
        let number = match bank.chars().nth(index) {
            Some(value) => value.to_digit(10).unwrap(),
            None => panic!("Not a valid digit"),
        };

        if max_right_side < number as i32 {
            max_right_side = number as i32;
        }
    }

    if max_left_side != -1 && max_right_side != -1 {
        let left = max_left_side * 10 + max;
        let right = max * 10 + max_right_side;
        if left > right { left } else { right }
    } else if max_left_side != -1 {
        max_left_side * 10 + max
    } else {
        max * 10 + max_right_side
    }
}
fn main() {
    let content_test = fs::read_to_string("test.txt").expect("Failed to load file");
    let content = fs::read_to_string("input.txt").expect("Failed to load file");

    let mut result = 0;
    let mut result_test = 0;

    for bank in content_test.split("\n") {
        if bank.is_empty() {
            continue;
        }
        let highest_joltage = find_highest_joltage(bank);
        println!("Bank: {}, highest joltage: {}", bank, highest_joltage);
        result_test += highest_joltage;
    }

    println!("Total output joltage (test): {}", result_test);

    assert!(result_test == 357);

    for bank in content.split("\n") {
        if bank.is_empty() {
            continue;
        }
        let highest_joltage = find_highest_joltage(bank);
        result += highest_joltage;
    }

    println!("Total output joltage: {}", result);

    println!("\nPART 2 -----------\n");

    let mut result = 0;
    let mut result_test = 0;

    for bank in content_test.split("\n") {
        if bank.is_empty() {
            continue;
        }
        let highest_joltage = find_highest_joltage_v2(bank);
        println!("Bank: {}, highest joltage: {}", bank, highest_joltage);
        result_test += highest_joltage;
    }

    println!("Total output joltage (test): {}", result_test);

    assert!(result_test == 3121910778619);

    for bank in content.split("\n") {
        if bank.is_empty() {
            continue;
        }
        let highest_joltage = find_highest_joltage_v2(bank);
        result += highest_joltage;
    }

    println!("Total output joltage: {}", result);
}
