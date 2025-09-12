// Day 262: Find all bridges in a connected undirected graph.
// Approach: Tarjan's bridge-finding algorithm using DFS with disc/low timestamps.
// An edge (u,v) is a bridge iff low[v] > disc[u]. Time O(V+E), Space O(V+E).

struct BridgeFinder {
    n: usize,
    adj: Vec<Vec<usize>>,
    timer: usize,
    disc: Vec<usize>,
    low: Vec<usize>,
    bridges: Vec<(usize, usize)>,
}

impl BridgeFinder {
    fn new(n: usize) -> Self {
        BridgeFinder {
            n,
            adj: vec![Vec::new(); n],
            timer: 0,
            disc: vec![0; n],
            low: vec![0; n],
            bridges: Vec::new(),
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.adj[v].push(u);
    }

    fn dfs(&mut self, u: usize, parent: i64) {
        self.timer += 1;
        self.disc[u] = self.timer;
        self.low[u] = self.timer;
        let mut skipped_parent = false;
        let neighbors = self.adj[u].clone();
        for v in neighbors {
            if v as i64 == parent && !skipped_parent {
                skipped_parent = true;
                continue;
            }
            if self.disc[v] == 0 {
                self.dfs(v, u as i64);
                self.low[u] = self.low[u].min(self.low[v]);
                if self.low[v] > self.disc[u] {
                    self.bridges.push((u.min(v), u.max(v)));
                }
            } else {
                self.low[u] = self.low[u].min(self.disc[v]);
            }
        }
    }

    fn find_bridges(&mut self) -> Vec<(usize, usize)> {
        for i in 0..self.n {
            if self.disc[i] == 0 {
                self.dfs(i, -1);
            }
        }
        self.bridges.sort();
        self.bridges.clone()
    }
}

fn main() {
    let mut g = BridgeFinder::new(5);
    for &(u, v) in &[(0, 1), (1, 2), (2, 0), (1, 3), (3, 4)] {
        g.add_edge(u, v);
    }
    let bridges = g.find_bridges();
    let parts: Vec<String> = bridges.iter().map(|(a, b)| format!("({}, {})", a, b)).collect();
    println!("Bridges: [{}]", parts.join(", "));
}
