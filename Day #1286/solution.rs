// Day 1286: Find all bridges in an undirected graph (Tarjan's low-link DFS).
// Time O(V + E), Space O(V + E).
struct Graph {
    adj: Vec<Vec<usize>>,
    disc: Vec<i32>,
    low: Vec<i32>,
    timer: i32,
    bridges: Vec<(usize, usize)>,
}

impl Graph {
    fn dfs(&mut self, u: usize, parent: i32) {
        self.timer += 1;
        self.disc[u] = self.timer;
        self.low[u] = self.timer;
        let mut skipped = false;
        let neighbors = self.adj[u].clone();
        for v in neighbors {
            if v as i32 == parent && !skipped {
                skipped = true;
                continue;
            }
            if self.disc[v] == 0 {
                self.dfs(v, u as i32);
                self.low[u] = self.low[u].min(self.low[v]);
                if self.low[v] > self.disc[u] {
                    self.bridges.push((u.min(v), u.max(v)));
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
    let mut adj = vec![Vec::new(); n];
    for &(u, v) in &edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut g = Graph { adj, disc: vec![0; n], low: vec![0; n], timer: 0, bridges: Vec::new() };
    for i in 0..n {
        if g.disc[i] == 0 {
            g.dfs(i, -1);
        }
    }
    g.bridges.sort();
    for (u, v) in &g.bridges {
        println!("{} - {}", u, v);
    }
}
