// Arbitrage = negative cycle in graph with weights -log(rate). Bellman-Ford.
// O(V^3) for a dense rate table.
fn has_arbitrage(rate: &Vec<Vec<f64>>) -> bool {
    let n = rate.len();
    let mut w = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        for j in 0..n {
            w[i][j] = -rate[i][j].ln();
        }
    }
    let mut dist = vec![0.0f64; n]; // virtual source, all 0
    for _ in 0..n - 1 {
        for u in 0..n {
            for v in 0..n {
                if dist[u] + w[u][v] < dist[v] {
                    dist[v] = dist[u] + w[u][v];
                }
            }
        }
    }
    for u in 0..n {
        for v in 0..n {
            if dist[u] + w[u][v] < dist[v] - 1e-12 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let rate = vec![
        vec![1.0, 0.8, 0.5],
        vec![1.3, 1.0, 1.9],
        vec![1.9, 0.5, 1.0],
    ];
    println!("{}", if has_arbitrage(&rate) { "true" } else { "false" }); // true
}
