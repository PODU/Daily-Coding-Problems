// PageRank: iterative power method with damping d=0.85, dangling-node mass redistributed evenly.
// Time O(iters*(N+E)), Space O(N+E).
use std::collections::HashMap;

fn main() {
    let nodes = vec!["A", "B", "C", "D"];
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    edges.insert("A", vec!["B", "C"]);
    edges.insert("B", vec!["C"]);
    edges.insert("C", vec!["A"]);
    edges.insert("D", vec!["C"]);
    let n = nodes.len() as f64;
    let d = 0.85;
    let mut score: HashMap<&str, f64> = nodes.iter().map(|&nd| (nd, 1.0 / n)).collect();

    for _ in 0..100 {
        let mut dangling = 0.0;
        for &nd in &nodes {
            if edges.get(nd).map_or(true, |o| o.is_empty()) {
                dangling += score[nd];
            }
        }
        let mut next: HashMap<&str, f64> = nodes
            .iter()
            .map(|&nd| (nd, (1.0 - d) / n + d * dangling / n))
            .collect();
        for &nd in &nodes {
            if let Some(outs) = edges.get(nd) {
                if outs.is_empty() {
                    continue;
                }
                let share = d * score[nd] / outs.len() as f64;
                for &t in outs {
                    *next.get_mut(t).unwrap() += share;
                }
            }
        }
        let diff: f64 = nodes.iter().map(|&nd| (next[nd] - score[nd]).abs()).sum();
        score = next;
        if diff < 1e-9 {
            break;
        }
    }

    let mut sorted = nodes.clone();
    sorted.sort();
    for nd in sorted {
        println!("{} {:.6}", nd, score[nd]);
    }
}
