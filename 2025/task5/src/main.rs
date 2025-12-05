use std::fs;

fn count_distinct_in_ranges(ranges: &[(u64, u64)]) -> u64 {
    if ranges.is_empty() {
        return 0;
    }

    let mut sorted = ranges.to_vec();
    sorted.sort_by_key(|r| r.0);

    let mut merged = vec![sorted[0]];

    for &(start, end) in &sorted[1..] {
        let last_index = merged.len() - 1;
        let (last_start, last_end) = merged[last_index];

        if start <= last_end + 1 {
            merged[last_index] = (last_start, last_end.max(end));
        } else {
            merged.push((start, end));
        }
    }

    merged.iter().map(|(s, e)| e - s + 1).sum()
}
fn solve(content: &str) -> (u64, u64) {
    let blocks: Vec<&str> = content.split("\n\n").collect();
    let mut counter = 0;

    let ranges: Vec<(u64, u64)> = blocks[0]
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse::<u64>().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            (start, end)
        })
        .collect();

    let count_distinct = count_distinct_in_ranges(&ranges);

    let ids: Vec<u64> = blocks[1]
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    for id in ids {
        for &(start, end) in &ranges {
            if (start..=end).contains(&id) {
                counter += 1;
                break;
            }
        }
    }

    (counter, count_distinct)
}
fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to load file");
    let content_test = fs::read_to_string("test.txt").expect("Failed to load file");

    let (result, result_p2) = solve(&content);
    let (result_test, result_test_p2) = solve(&content_test);

    println!("Fresh ingredients (test) {}", result_test);
    assert!(result_test == 3);

    println!("Fresh ingredients {}", result);

    println!("PART2 --------------\n");

    println!("Fresh ingredients (test) {}", result_test_p2);
    assert!(result_test_p2 == 14);

    println!("Fresh ingredients {}", result_p2);
}
