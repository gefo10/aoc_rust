use std::cmp::{Reverse, max, min};
use std::fs;
#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
}

struct Rectangle {
    p1: Point,
    p2: Point,
    area: i64,
}

struct Segment {
    p1: Point,
    p2: Point,
    // Helper to know orientation
    is_vertical: bool,
}

fn get_points(content: &str) -> Vec<Point> {
    let points: Vec<Point> = content
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            let x = parts.next().unwrap().trim().parse().unwrap();
            let y = parts.next().unwrap().trim().parse().unwrap();

            Point { x, y }
        })
        .collect();

    points
}
fn generate_rectangles(points: &Vec<Point>) -> Vec<Rectangle> {
    let mut rectangles = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let ipoint = &points[i];
            let jpoint = &points[j];

            if ipoint.x == jpoint.x || ipoint.y == jpoint.y {
                continue;
            }

            let width = if ipoint.x > jpoint.x {
                ipoint.x - jpoint.x
            } else {
                jpoint.x - ipoint.x
            } + 1;

            let height = if ipoint.y > jpoint.y {
                ipoint.y - jpoint.y
            } else {
                jpoint.y - ipoint.y
            } + 1;
            rectangles.push(Rectangle {
                p1: ipoint.clone(),
                p2: jpoint.clone(),
                area: width * height,
            });
        }
    }

    rectangles.sort_unstable_by_key(|r| Reverse(r.area));

    rectangles
}

// Check if a value 'val' is strictly between 'a' and 'b'
fn is_between(val: i64, a: i64, b: i64) -> bool {
    let min_v = min(a, b);
    let max_v = max(a, b);
    val > min_v && val < max_v
}

// Check if two ranges (a1..a2) and (b1..b2) overlap
// We are checking for STRICT overlap of the intervals
fn ranges_overlap(a1: i64, a2: i64, b1: i64, b2: i64) -> bool {
    let start_a = min(a1, a2);
    let end_a = max(a1, a2);
    let start_b = min(b1, b2);
    let end_b = max(b1, b2);

    // Standard range overlap logic: max(starts) < min(ends)
    max(start_a, start_b) < min(end_a, end_b)
}

fn is_valid_rectangle(r: &Rectangle, segments: &Vec<Segment>) -> bool {
    let rx_min = min(r.p1.x, r.p2.x);
    let rx_max = max(r.p1.x, r.p2.x);
    let ry_min = min(r.p1.y, r.p2.y);
    let ry_max = max(r.p1.y, r.p2.y);

    // 1. Edge Intersection Check (Does a wall cut through the room?)
    for seg in segments {
        if seg.is_vertical {
            // Vertical Wall at x = seg.p1.x
            // It cuts if x is inside (rx_min, rx_max) AND y-ranges overlap
            if is_between(seg.p1.x, rx_min, rx_max)
                && ranges_overlap(seg.p1.y, seg.p2.y, ry_min, ry_max)
            {
                return false;
            }
        } else {
            // Horizontal Wall at y = seg.p1.y
            // It cuts if y is inside (ry_min, ry_max) AND x-ranges overlap
            if is_between(seg.p1.y, ry_min, ry_max)
                && ranges_overlap(seg.p1.x, seg.p2.x, rx_min, rx_max)
            {
                return false;
            }
        }
    }

    // 2. Center Point Check (Are we floating in empty space?)
    // We use f64 for the center to handle half-coordinates cleanly
    let cx = (rx_min as f64 + rx_max as f64) / 2.0;
    let cy = (ry_min as f64 + ry_max as f64) / 2.0;

    // Ray Casting Algorithm: Count intersections with vertical edges to the right
    let mut intersections = 0;
    for seg in segments {
        if seg.is_vertical {
            // Check if our Y is within the segment's Y range
            // Standard Ray Cast rule: include one endpoint, exclude the other to avoid double counting
            // We'll use y_min <= cy < y_max
            let sy_min = min(seg.p1.y, seg.p2.y) as f64;
            let sy_max = max(seg.p1.y, seg.p2.y) as f64;

            if cy >= sy_min && cy < sy_max {
                // Check if the edge is to the right of our point
                if (seg.p1.x as f64) > cx {
                    intersections += 1;
                }
            }
        }
    }

    // Inside if ODD number of intersections
    intersections % 2 != 0
}

fn solve_part2(points: &Vec<Point>) -> i64 {
    // 1. Build Polygon Segments
    let mut segments = Vec::new();
    for i in 0..points.len() {
        let p1 = &points[i];
        let p2 = &points[(i + 1) % points.len()]; // Wrap around to 0
        segments.push(Segment {
            p1: p1.clone(),
            p2: p2.clone(),
            is_vertical: p1.x == p2.x,
        });
    }

    // 2. Generate Rectangles
    let mut rectangles = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];

            //if p1.x == p2.x || p1.y == p2.y {
            //    continue;
            //}

            let width = (p1.x - p2.x).abs() + 1;
            let height = (p1.y - p2.y).abs() + 1;

            rectangles.push(Rectangle {
                p1: p1.clone(),
                p2: p2.clone(),
                area: width * height,
            });
        }
    }

    // 3. Sort by Area Descending (Optimization)
    rectangles.sort_unstable_by_key(|r| Reverse(r.area));

    // 4. Find first valid
    for rect in rectangles {
        if is_valid_rectangle(&rect, &segments) {
            return rect.area;
        }
    }

    0
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let content_test = fs::read_to_string("test.txt").unwrap();

    let points_test = get_points(&content_test);
    let rectangles_test = generate_rectangles(&points_test);
    println!("Largest area is (test): {}", rectangles_test[0].area);
    assert!(rectangles_test[0].area == 50);

    let points = get_points(&content);
    let rectangles = generate_rectangles(&points);
    println!("Largest area is (test): {}", rectangles[0].area);

    println!("Part 2 -------------------");
    let res_test = solve_part2(&points_test);
    println!("Largest area is (test): {}", res_test);

    assert!(res_test == 24);

    let res = solve_part2(&points);
    println!("Largest area is: {}", res);
}
