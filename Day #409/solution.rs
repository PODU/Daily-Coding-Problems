// Day 409: PageRank via power iteration with damping d=0.85.
// Approach: iterate score(j)=(1-d)/N + d*sum(score(i)/Ci); dangling rank
// is spread over all nodes. Time: O(iters*(N+E)), Space: O(N+E).
use std::collections::BTreeMap;

fn page_rank(
    graph: &BTreeMap<&str, Vec<&str>>,
    d: f64,
    iters: usize,
    eps: f64,
) -> BTreeMap<String, f64> {
    let n = graph.len() as f64;
    let mut rank: BTreeMap<String, f64> =
        graph.keys().map(|&k| (k.to_string(), 1.0 / n)).collect();
    for _ in 0..iters {
        // Dangling nodes (no out-links) leak rank; redistribute it evenly.
        let dangling: f64 = graph
            .iter()
            .filter(|(_, outs)| outs.is_empty())
            .map(|(k, _)| rank[*k])
            .sum();
        let mut next: BTreeMap<String, f64> = graph
            .keys()
            .map(|&k| (k.to_string(), (1.0 - d) / n + d * dangling / n))
            .collect();
        for (node, outs) in graph {
            if outs.is_empty() {
                continue;
            }
            let share = rank[*node] / outs.len() as f64;
            for nbr in outs {
                *next.get_mut(*nbr).unwrap() += d * share;
            }
        }
        let diff: f64 = graph
            .keys()
            .map(|&k| (next[k] - rank[k]).abs())
            .sum();
        rank = next;
        if diff < eps {
            break;
        }
    }
    rank
}

fn main() {
    let mut graph: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    graph.insert("A", vec!["B", "C"]);
    graph.insert("B", vec!["C"]);
    graph.insert("C", vec!["A"]);
    let rank = page_rank(&graph, 0.85, 100, 1e-12);
    for (node, score) in &rank {
        println!("{}: {:.4}", node, score);
    }
}
