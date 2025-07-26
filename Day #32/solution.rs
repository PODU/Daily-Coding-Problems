// Currency arbitrage: weights = -log(rate), Bellman-Ford detects negative cycle. O(V*E).
fn has_arbitrage(rates: &Vec<Vec<f64>>) -> bool {
    let n = rates.len();
    let w: Vec<Vec<f64>> = rates
        .iter()
        .map(|row| row.iter().map(|x| -x.ln()).collect())
        .collect();
    let mut dist = vec![0.0f64; n]; // virtual super-source reaching all nodes at 0
    for _ in 0..n - 1 {
        for u in 0..n {
            for v in 0..n {
                if dist[u] + w[u][v] < dist[v] - 1e-12 {
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
    let r1 = vec![
        vec![1.0, 0.7, 0.5],
        vec![1.4, 1.0, 0.7],
        vec![2.1, 1.4, 1.0],
    ];
    let r2 = vec![
        vec![1.0, 2.0, 4.0],
        vec![0.5, 1.0, 2.0],
        vec![0.25, 0.5, 1.0],
    ];
    println!("{}", has_arbitrage(&r1));
    println!("{}", has_arbitrage(&r2));
}
