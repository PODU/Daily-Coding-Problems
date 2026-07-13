// Day 1805: Find all bridges via Tarjan's algorithm (disc/low, edge is bridge if low[v] > disc[u]).
// Parallel edges handled by skipping the parent edge once via edge id. O(V+E).
struct Graph {
    adj: Vec<Vec<(usize, usize)>>, // (neighbor, edge id)
    disc: Vec<i32>,
    low: Vec<i32>,
    timer: i32,
    result: Vec<(usize, usize)>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph { adj: vec![Vec::new(); n], disc: vec![-1; n], low: vec![-1; n], timer: 0, result: Vec::new() }
    }

    fn add_edge(&mut self, u: usize, v: usize, id: usize) {
        self.adj[u].push((v, id));
        self.adj[v].push((u, id));
    }

    fn dfs(&mut self, u: usize, parent_edge: i64) {
        self.disc[u] = self.timer;
        self.low[u] = self.timer;
        self.timer += 1;
        for i in 0..self.adj[u].len() {
            let (v, id) = self.adj[u][i];
            if id as i64 == parent_edge {
                continue; // skip the single parent edge once
            }
            if self.disc[v] == -1 {
                self.dfs(v, id as i64);
                self.low[u] = self.low[u].min(self.low[v]);
                if self.low[v] > self.disc[u] {
                    self.result.push((u.min(v), u.max(v)));
                }
            } else {
                self.low[u] = self.low[u].min(self.disc[v]);
            }
        }
    }
}

fn main() {
    let n = 5;
    let edges = [(0, 1), (1, 2), (2, 0), (1, 3), (3, 4)];
    let mut g = Graph::new(n);
    for (i, &(u, v)) in edges.iter().enumerate() {
        g.add_edge(u, v, i);
    }
    for i in 0..n {
        if g.disc[i] == -1 {
            g.dfs(i, -1);
        }
    }
    g.result.sort();
    for (u, v) in &g.result {
        println!("{} - {}", u, v);
    }
    // expected: 1 - 3 and 3 - 4
}
