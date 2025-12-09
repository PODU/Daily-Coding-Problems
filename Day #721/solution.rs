// Day 721: Maximum-weight spanning tree.
// Approach: Kruskal with edges sorted by DECREASING weight + union-find.
// Time: O(E log E), Space: O(V + E).

struct Dsu { parent: Vec<usize>, rank: Vec<usize> }
impl Dsu {
    fn new(n: usize) -> Self { Dsu { parent: (0..n).collect(), rank: vec![0; n] } }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x { let r = self.find(self.parent[x]); self.parent[x] = r; }
        self.parent[x]
    }
    fn unite(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b { return false; }
        if self.rank[a] < self.rank[b] { std::mem::swap(&mut a, &mut b); }
        self.parent[b] = a;
        if self.rank[a] == self.rank[b] { self.rank[a] += 1; }
        true
    }
}

fn max_spanning_tree(n: usize, mut edges: Vec<(usize, usize, i64)>) -> i64 {
    edges.sort_by(|a, b| b.2.cmp(&a.2));
    let mut dsu = Dsu::new(n);
    let (mut total, mut used) = (0i64, 0usize);
    for (u, v, w) in edges {
        if dsu.unite(u, v) { total += w; used += 1; }
    }
    if used == n - 1 { total } else { -1 }
}

fn main() {
    let n = 4;
    let edges = vec![(0, 1, 1), (0, 2, 2), (0, 3, 3), (1, 2, 4), (2, 3, 5)];
    println!("Maximum spanning tree weight: {}", max_spanning_tree(n, edges));
}
