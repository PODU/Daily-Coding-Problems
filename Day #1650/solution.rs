// Maximum spanning tree: Kruskal with edges sorted DESC by weight + Union-Find (path compression, union by rank). O(E log E).
struct Dsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu { parent: (0..n).collect(), rank: vec![0; n] }
    }
    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }
    fn unite(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        if self.rank[a] < self.rank[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        if self.rank[a] == self.rank[b] {
            self.rank[a] += 1;
        }
        true
    }
}

fn max_spanning_tree(n: usize, mut edges: Vec<(usize, usize, i32)>) -> i32 {
    edges.sort_by(|a, b| b.2.cmp(&a.2));
    let mut dsu = Dsu::new(n);
    let mut total = 0;
    for (u, v, w) in edges {
        if dsu.unite(u, v) {
            total += w;
        }
    }
    total
}

fn main() {
    let edges = vec![(0, 1, 1), (0, 2, 2), (0, 3, 3), (1, 2, 4), (2, 3, 5)];
    println!("Maximum spanning tree weight: {}", max_spanning_tree(4, edges));
}
