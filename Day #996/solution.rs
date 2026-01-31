// Day 996: Maximum weight spanning tree.
// Kruskal's algorithm with edges sorted in DESCENDING weight + union-find.
// O(E log E) time, O(V) space.
struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu { parent: (0..n).collect() }
    }
    fn find(&mut self, x: usize) -> usize {
        let mut x = x;
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        self.parent[ra] = rb;
        true
    }
}

fn main() {
    let n = 4;
    // (weight, u, v)
    let mut edges = vec![(1, 0, 1), (3, 0, 2), (2, 1, 2), (4, 1, 3), (5, 2, 3)];
    edges.sort_by(|a, b| b.0.cmp(&a.0)); // heaviest first
    let mut dsu = Dsu::new(n);
    let mut total = 0;
    for (w, u, v) in edges {
        if dsu.union(u, v) {
            total += w;
        }
    }
    println!("Maximum spanning tree weight: {}", total); // 12
}
