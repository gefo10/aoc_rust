use std::{cmp::Ordering, fs};

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to open file");

    let mut count = 0;
    for &line in &content.lines().collect::<Vec<&str>>() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        if check_monotonic(&numbers) {
            count += 1;
        } else {
            for i in 0..numbers.len() {
                let mut temp = numbers.to_vec();
                temp.remove(i);
                if check_monotonic(&temp) {
                    count += 1;
                    break;
                }
            }
        }
    }

    println!("Answer is: {}", count);
}

fn check_monotonic(numbers: &[i32]) -> bool {
    let mut direction: Option<Ordering> = None;

    if numbers.len() < 1 {
        return false;
    }

    for window in numbers.windows(2) {
        let current = window[1].cmp(&window[0]);
        let diff = (window[1] - window[0]).abs();

        if !(diff >= 1 && diff <= 3) {
            return false;
        }

        if current == Ordering::Equal {
            //println!("Equal ordering");
            return false;
        }

        match direction {
            None => direction = Some(current),
            Some(expected) if expected != current => return false,
            _ => {}
        }
    }

    true
}

fn check_monotonic2(numbers: &[i32]) -> bool {
    let mut direction: Option<Ordering> = None;

    if numbers.len() < 1 {
        return false;
    }

    let mut i = 0;
    while i < numbers.len() - 1 {
        let first = numbers[i];
        let second = numbers[i + 1];
        let current = first.cmp(&second);

        let diff = (first - second).abs();

        if !(diff >= 1 && diff <= 3) {
            //try 2 checks:
            // -- one without first
            // -- one without second

            let mut temp_first = numbers.to_vec();
            temp_first.remove(i);

            let mut temp_second = numbers.to_vec();
            temp_second.remove(i + 1);
            return check_monotonic(&temp_first) || check_monotonic(&temp_second);
        }

        if current == Ordering::Equal {
            //println!("Equal ordering");
            let mut temp_first = numbers.to_vec();
            temp_first.remove(i);

            return check_monotonic(&temp_first);
        }

        match direction {
            None => direction = Some(current),
            Some(expected) if expected != current => {
                let mut temp_first = numbers.to_vec();
                temp_first.remove(i);

                let mut temp_second = numbers.to_vec();
                temp_second.remove(i + 1);
                return check_monotonic(&temp_first) || check_monotonic(&temp_second);
            }
            _ => {}
        }

        i += 1;
    }

    true
}
