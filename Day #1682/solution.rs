// Day 1682: Graph k-colorability via backtracking with pruning.
// Time O(k^V) worst case, Space O(V).
fn solve(adj: &[Vec<i32>], color: &mut Vec<i32>, k: i32, v: usize) -> bool {
    let n = adj.len();
    if v == n {
        return true;
    }
    for c in 0..k {
        let mut ok = true;
        for u in 0..n {
            if adj[v][u] == 1 && color[u] == c {
                ok = false;
                break;
            }
        }
        if ok {
            color[v] = c;
            if solve(adj, color, k, v + 1) {
                return true;
            }
            color[v] = -1;
        }
    }
    false
}

fn can_color(adj: &[Vec<i32>], k: i32) -> bool {
    let mut color = vec![-1; adj.len()];
    solve(adj, &mut color, k, 0)
}

fn main() {
    let tri = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
    println!("{}", can_color(&tri, 2)); // false
    println!("{}", can_color(&tri, 3)); // true
}
