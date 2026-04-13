// Arbitrage detection: edge weight = -log(rate); negative cycle => arbitrage. Bellman-Ford.
// Time: O(V*E) = O(V^3), Space: O(V).
fn has_arbitrage(rates: &[Vec<f64>]) -> bool {
    let n = rates.len();
    let mut dist = vec![0.0f64; n]; // all 0: detect any reachable negative cycle
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
    let rates = vec![
        vec![1.0, 2.0, 1.0],
        vec![0.5, 1.0, 4.0],
        vec![1.0, 0.25, 1.0],
    ];
    println!("Arbitrage exists: {}", has_arbitrage(&rates));
}
