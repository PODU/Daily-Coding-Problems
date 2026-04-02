// Day 1289: PageRank via power iteration with damping factor d.
// Iterate score vector until convergence; dangling nodes spread mass uniformly.
// Time O(iters * (V + E)), Space O(V + E).
use std::collections::HashMap;

fn main() {
    let nodes = ["A", "B", "C", "D"];
    let mut links: HashMap<&str, Vec<&str>> = HashMap::new();
    links.insert("A", vec!["B", "C"]);
    links.insert("B", vec!["C"]);
    links.insert("C", vec!["A"]);
    links.insert("D", vec!["C"]);
    let d = 0.85;
    let iters = 100;
    let n = nodes.len() as f64;

    let mut out_count: HashMap<&str, usize> = HashMap::new();
    for &node in &nodes {
        out_count.insert(node, links.get(node).map_or(0, |v| v.len()));
    }
    let mut score: HashMap<&str, f64> = HashMap::new();
    for &node in &nodes {
        score.insert(node, 1.0 / n);
    }

    for _ in 0..iters {
        let dangling_sum: f64 = nodes.iter().filter(|&&node| out_count[node] == 0).map(|&node| score[node]).sum();
        let mut nw: HashMap<&str, f64> = HashMap::new();
        for &node in &nodes {
            nw.insert(node, (1.0 - d) / n + d * dangling_sum / n);
        }
        for &src in &nodes {
            if out_count[src] == 0 {
                continue;
            }
            let share = d * score[src] / out_count[src] as f64;
            for &dst in &links[src] {
                *nw.get_mut(dst).unwrap() += share;
            }
        }
        score = nw;
    }
    for &node in &nodes {
        println!("{}: {:.4}", node, score[node]);
    }
}
