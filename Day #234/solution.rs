// Maximum spanning tree: Kruskal with edges sorted by descending weight + union-find.
// Time: O(E log E), Space: O(V).
struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU { parent: (0..n).collect() }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let r = self.find(self.parent[x]);
            self.parent[x] = r;
        }
        self.parent[x]
    }
    fn unite(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        self.parent[ra] = rb;
        true
    }
}

fn max_spanning_tree(n: usize, mut edges: Vec<(usize, usize, i32)>) -> (i32, Vec<(usize, usize)>) {
    edges.sort_by(|a, b| b.2.cmp(&a.2));
    let mut dsu = DSU::new(n);
    let mut total = 0;
    let mut chosen = vec![];
    for (u, v, w) in edges {
        if dsu.unite(u, v) {
            total += w;
            chosen.push((u, v));
        }
    }
    (total, chosen)
}

fn main() {
    let n = 4;
    let edges = vec![(0, 1, 1), (1, 2, 2), (2, 3, 3), (0, 3, 4), (0, 2, 5)];
    let (total, chosen) = max_spanning_tree(n, edges);
    println!("Max spanning tree weight: {}", total); // 11
    let parts: Vec<String> = chosen.iter().map(|(u, v)| format!("({}-{})", u, v)).collect();
    println!("Edges: {}", parts.join(" "));
}
