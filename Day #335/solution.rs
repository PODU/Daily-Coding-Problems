// PageRank via iterative power method, d=0.85. Iterate until convergence.
// Time: O(iters * (N + E)). Space: O(N + E).
use std::collections::HashMap;

fn main() {
    let names = ["A", "B", "C", "D"];
    let n = names.len();
    let mut idx = HashMap::new();
    for (i, name) in names.iter().enumerate() {
        idx.insert(*name, i);
    }
    let edges = [("A", "B"), ("A", "C"), ("B", "C"), ("C", "A"), ("D", "C")];
    let mut incoming: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut out = vec![0usize; n];
    for (u, v) in edges.iter() {
        incoming[idx[v]].push(idx[u]);
        out[idx[u]] += 1;
    }

    let d = 0.85;
    let mut score = vec![1.0 / n as f64; n];
    for _ in 0..1000 {
        let mut nxt = vec![(1.0 - d) / n as f64; n];
        for j in 0..n {
            for &i in &incoming[j] {
                nxt[j] += d * score[i] / out[i] as f64;
            }
        }
        let diff: f64 = (0..n).map(|k| (nxt[k] - score[k]).abs()).sum();
        score = nxt;
        if diff < 1e-9 {
            break;
        }
    }
    for i in 0..n {
        println!("{}: {:.4}", names[i], score[i]);
    }
}
