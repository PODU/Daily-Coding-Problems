// PageRank via power iteration. Dangling nodes distribute rank evenly.
// Time O(iters * (N+E)), Space O(N+E).
fn pagerank(out: &[Vec<usize>], d: f64, max_iter: usize, tol: f64) -> Vec<f64> {
    let n = out.len();
    let mut rank = vec![1.0 / n as f64; n];
    for _ in 0..max_iter {
        let mut next = vec![(1.0 - d) / n as f64; n];
        let mut dangling = 0.0;
        for i in 0..n {
            if out[i].is_empty() {
                dangling += rank[i];
            }
        }
        for i in 0..n {
            if out[i].is_empty() {
                continue;
            }
            let share = rank[i] / out[i].len() as f64;
            for &j in &out[i] {
                next[j] += d * share;
            }
        }
        for j in 0..n {
            next[j] += d * dangling / n as f64;
        }
        let diff: f64 = (0..n).map(|j| (next[j] - rank[j]).abs()).sum();
        rank = next;
        if diff < tol {
            break;
        }
    }
    rank
}

fn main() {
    let out: Vec<Vec<usize>> = vec![vec![1, 2], vec![2], vec![0, 1], vec![0, 1, 2]];
    let rank = pagerank(&out, 0.85, 1000, 1e-9);
    for (i, r) in rank.iter().enumerate() {
        println!("{}: {:.4}", i, r);
    }
}
