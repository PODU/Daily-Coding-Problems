// Arbitrage detection: weight=-log(rate), Bellman-Ford finds a negative-weight cycle => arbitrage. O(V^3) (V*E edges, V-1 passes).

fn has_arbitrage(rates: &[Vec<f64>]) -> bool {
    let n = rates.len();
    let mut dist = vec![0.0f64; n]; // virtual source: all start at 0
    for _ in 0..n - 1 {
        for u in 0..n {
            for v in 0..n {
                let w = -rates[u][v].ln();
                if dist[u] + w < dist[v] - 1e-12 {
                    dist[v] = dist[u] + w;
                }
            }
        }
    }
    for u in 0..n {
        for v in 0..n {
            let w = -rates[u][v].ln();
            if dist[u] + w < dist[v] - 1e-12 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let arb = vec![vec![1.0, 0.5, 0.2], vec![2.0, 1.0, 0.5], vec![5.0, 2.0, 1.0]];
    let consistent = vec![vec![1.0, 2.0, 4.0], vec![0.5, 1.0, 2.0], vec![0.25, 0.5, 1.0]];
    println!("{}", if has_arbitrage(&arb) { "true" } else { "false" });
    println!("{}", if has_arbitrage(&consistent) { "true" } else { "false" });
}
