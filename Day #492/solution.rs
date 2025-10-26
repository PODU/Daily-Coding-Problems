// Day 492: Graph m-colorability via backtracking.
// Assign each vertex a color 1..k, ensuring no adjacent pair (adjacency matrix) matches.
// Time: O(k^V) worst case, Space: O(V) for the color assignment + recursion stack.

fn is_safe(v: usize, graph: &[Vec<i32>], colors: &[i32], c: i32) -> bool {
    for u in 0..graph.len() {
        if graph[v][u] == 1 && colors[u] == c {
            return false;
        }
    }
    true
}

fn solve(v: usize, graph: &[Vec<i32>], k: i32, colors: &mut Vec<i32>) -> bool {
    if v == graph.len() {
        return true;
    }
    for c in 1..=k {
        if is_safe(v, graph, colors, c) {
            colors[v] = c;
            if solve(v + 1, graph, k, colors) {
                return true;
            }
            colors[v] = 0;
        }
    }
    false
}

fn can_color(graph: &[Vec<i32>], k: i32) -> bool {
    let mut colors = vec![0; graph.len()];
    solve(0, graph, k, &mut colors)
}

fn main() {
    // Triangle K3: every pair adjacent.
    let graph = vec![
        vec![0, 1, 1],
        vec![1, 0, 1],
        vec![1, 1, 0],
    ];
    println!("k=2 colorable: {}", can_color(&graph, 2));
    println!("k=3 colorable: {}", can_color(&graph, 3));
}
