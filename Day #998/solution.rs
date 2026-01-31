// Day 998: Graph k-colorability (adjacency matrix).
// Backtracking: try each color per vertex, skipping colors used by neighbors.
// O(k^V) worst case, O(V) extra space.
fn can_color(graph: &[Vec<i32>], k: i32) -> bool {
    let n = graph.len();
    let mut colors = vec![0i32; n];

    fn ok(graph: &[Vec<i32>], colors: &[i32], n: usize, v: usize, c: i32) -> bool {
        (0..n).all(|u| !(graph[v][u] == 1 && colors[u] == c))
    }

    fn solve(graph: &[Vec<i32>], colors: &mut [i32], n: usize, k: i32, v: usize) -> bool {
        if v == n {
            return true;
        }
        for c in 1..=k {
            if ok(graph, colors, n, v, c) {
                colors[v] = c;
                if solve(graph, colors, n, k, v + 1) {
                    return true;
                }
                colors[v] = 0;
            }
        }
        false
    }

    solve(graph, &mut colors, n, k, 0)
}

fn main() {
    let triangle = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
    println!("{}", can_color(&triangle, 2)); // false
    println!("{}", can_color(&triangle, 3)); // true
}
