use std::cmp::{Reverse, min};
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Edge {
    u: usize,
    v: usize,
    dist_sq: i64,
}

// --- Union-Find Struct ---
struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        let root = self.find(self.parent[i]);
        self.parent[i] = root;
        root
    }

    // Returns TRUE if a merge happened (they were separate), FALSE if already connected
    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            self.parent[root_j] = root_i;
            true // Successfully merged
        } else {
            false // Already in same group
        }
    }
}

impl Point {
    fn dist_sq(&self, other: &Point) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

// --- Shared Parsing & Edge Generation ---
fn parse_and_get_sorted_edges(content: &str) -> (Vec<Point>, Vec<Edge>) {
    let points: Vec<Point> = content
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            Point {
                x: parts.next().unwrap().trim().parse().unwrap(),
                y: parts.next().unwrap().trim().parse().unwrap(),
                z: parts.next().unwrap().trim().parse().unwrap(),
            }
        })
        .collect();

    let mut edges = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            edges.push(Edge {
                u: i,
                v: j,
                dist_sq: points[i].dist_sq(&points[j]),
            });
        }
    }

    // Sort closest first
    edges.sort_unstable_by_key(|e| e.dist_sq);

    (points, edges)
}

fn solve_part1(points: &Vec<Point>, edges: &Vec<Edge>, limit_count: usize) -> usize {
    let mut uf = UnionFind::new(points.len());
    // Use sizes only for Part 1 logic
    let mut sizes = vec![1; points.len()];

    let limit = min(limit_count, edges.len());

    for i in 0..limit {
        let u = edges[i].u;
        let v = edges[i].v;

        let root_u = uf.find(u);
        let root_v = uf.find(v);

        if root_u != root_v {
            uf.union(u, v);
            // We need to manually track sizes here since we simplified the UF struct
            // Note: After union(u, v), root_v becomes child of root_u (based on my impl above)
            // But we need to be careful with the order. Let's just redo the logic safely:
            // Re-find roots to be sure who merged into whom
            let new_root = uf.find(u);
            // The one that is NOT new_root was merged.
            // But since we are inside a specific logic block, let's just stick to the UF behavior:
            // My `union` implementation makes `root_j` (v) point to `root_i` (u).
            sizes[root_u] += sizes[root_v];
        }
    }

    let mut valid_sizes = Vec::new();
    for i in 0..points.len() {
        if uf.parent[i] == i {
            valid_sizes.push(sizes[i]);
        }
    }
    valid_sizes.sort_unstable_by_key(|&w| Reverse(w));
    valid_sizes.iter().take(3).product()
}

fn solve_part2(points: &Vec<Point>, edges: &Vec<Edge>) -> i64 {
    let mut uf = UnionFind::new(points.len());
    let mut groups_count = points.len();

    for edge in edges {
        // Try to connect the two points
        if uf.union(edge.u, edge.v) {
            // If they were separate, they are now one. Count decreases.
            groups_count -= 1;

            // If we are down to 1 group, EVERYONE is connected.
            if groups_count == 1 {
                println!("Final connection between index {} and {}", edge.u, edge.v);
                return points[edge.u].x * points[edge.v].x;
            }
        }
    }
    0
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("input.txt missing");

    // Parse once
    let (points, edges) = parse_and_get_sorted_edges(&content);

    // Solve Part 1
    let part1 = solve_part1(&points, &edges, 1000);
    println!("Part 1 Answer: {}", part1);

    // Solve Part 2
    let part2 = solve_part2(&points, &edges);
    println!("Part 2 Answer: {}", part2);
}
