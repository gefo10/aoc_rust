use std::fs;

fn count_zeros_right(start: u8, clicks: i32) -> i32 {
    let mut count = 0;
    let mut pos = start;

    for _ in 0..clicks {
        pos = (pos + 1) % 100;
        if pos == 0 {
            count += 1;
        }
    }

    count
}

fn count_zeros_left(start: u8, clicks: i32) -> i32 {
    let mut count = 0;
    let mut pos = start;

    for _ in 0..clicks {
        pos = if pos == 0 { 99 } else { pos - 1 };
        if pos == 0 {
            count += 1;
        }
    }

    count
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let mut dial: u8 = 50;
    let mut counter = 0;

    for line in content.lines() {
        if line.is_empty() {
            continue;
        }

        let dir = line.chars().nth(0).unwrap();
        let value = str::parse::<i32>(&line[1..]).unwrap();

        match dir {
            'R' => {
                counter += count_zeros_right(dial, value);
                dial = ((dial as i32 + value) % 100) as u8;
            }
            'L' => {
                counter += count_zeros_left(dial, value);
                dial = ((dial as i32 - value).rem_euclid(100)) as u8;
            }
            _ => panic!("Not valid"),
        }
    }

    println!("{}", counter);
}
