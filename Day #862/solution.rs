// Day 862: Find all bridges in a connected undirected graph.
// Approach: Tarjan's DFS using discovery times and low-link values.
// Time: O(V + E), Space: O(V + E).
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
        let neighbors = self.adj[u].clone();
        for v in neighbors {
            if v as i32 == parent {
                continue;
            }
            if self.disc[v] != 0 {
                self.low[u] = self.low[u].min(self.disc[v]);
            } else {
                self.dfs(v, u as i32);
                self.low[u] = self.low[u].min(self.low[v]);
                if self.low[v] > self.disc[u] {
                    self.bridges.push((u.min(v), u.max(v)));
                }
            }
        }
    }
}

fn main() {
    let n = 5;
    let edges = [(0, 1), (1, 2), (2, 0), (1, 3), (3, 4)];
    let mut adj = vec![Vec::new(); n];
    for &(a, b) in &edges {
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut g = Graph { adj, disc: vec![0; n], low: vec![0; n], timer: 0, bridges: Vec::new() };
    for i in 0..n {
        if g.disc[i] == 0 {
            g.dfs(i, -1);
        }
    }
    g.bridges.sort();
    let parts: Vec<String> = g.bridges.iter().map(|(a, b)| format!("({}, {})", a, b)).collect();
    println!("Bridges: {}", parts.join(" ")); // (1, 3) (3, 4)
}
