// Day 941: Arbitrage exists iff a cycle has product of rates > 1, i.e. a negative cycle
// under weights w = -log(rate). Bellman-Ford. Time O(V^3), Space O(V).
fn has_arbitrage(rate: &[Vec<f64>]) -> bool {
    let n = rate.len();
    let mut dist = vec![0.0f64; n]; // virtual source connected to all with weight 0
    for _ in 0..n - 1 {
        for u in 0..n {
            for v in 0..n {
                let w = -rate[u][v].ln();
                if dist[u] + w < dist[v] - 1e-12 {
                    dist[v] = dist[u] + w;
                }
            }
        }
    }
    for u in 0..n {
        for v in 0..n {
            let w = -rate[u][v].ln();
            if dist[u] + w < dist[v] - 1e-12 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let rate = vec![
        vec![1.0, 2.0, 1.0],
        vec![0.5, 1.0, 2.0],
        vec![1.0, 0.5, 1.0],
    ];
    println!("{}", has_arbitrage(&rate)); // true (0->1->2->0 = 4 > 1)
}
