use std::fs;

fn is_valid_id(val: &str) -> bool {
    let len = val.len();

    let pattern = &val[0..len / 2];

    if pattern.repeat(2) == val {
        return false;
    }

    true
}

fn is_valid_id_part2(val: &str) -> bool {
    let len = val.len();

    for pattern_len in 1..=len / 2 {
        let pattern = &val[0..pattern_len];

        if pattern.repeat(len / pattern_len) == val {
            return false;
        }
    }

    true
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("failed to open");

    let mut result = 0;
    let mut result_part2 = 0;

    for range in content.split(',') {
        print!("Range is {}\n", range);
        let (start, end): (u64, u64) = {
            let (s, e) = match range.split_once('-') {
                Some(pair) => pair,
                None => {
                    eprintln!("ERROR: No dash found in range: '{}'", range);
                    continue;
                }
            };

            println!("Attempting to parse start: '{}'", s);
            let start: u64 = match s.trim().parse() {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("ERROR parsing start '{}': {:?}", s, e);
                    eprintln!("Bytes: {:?}", s.as_bytes());
                    panic!("Failed");
                }
            };

            println!("Attempting to parse end: '{}'", e);
            let end: u64 = match e.trim().parse() {
                Ok(val) => val,
                Err(err) => {
                    eprintln!("ERROR parsing end '{}': {:?}", e, err);
                    eprintln!("Bytes: {:?}", e.as_bytes());
                    panic!("Failed");
                }
            };

            (start, end)
        };

        for value in start..=end {
            if !is_valid_id(&value.to_string()) {
                //println!("Invalid ID: {}", value);
                result += value;
            }

            if !is_valid_id_part2(&value.to_string()) {
                println!("Invalid ID: {}", value);
                result_part2 += value;
            }
        }
    }

    println!("Sum of all invalid IDs: {}", result);
    println!("Sum of all invalid IDs (part 2): {}", result_part2);
}
