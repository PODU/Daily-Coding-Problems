// Day 1246: Maximum weight spanning tree.
// Approach: Kruskal's algorithm with edges sorted in DESCENDING weight + union-find.
// Time: O(E log E), Space: O(V + E).

struct Dsu {
    p: Vec<usize>,
    r: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu { p: (0..n).collect(), r: vec![0; n] }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    fn unite(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        if self.r[a] < self.r[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.p[b] = a;
        if self.r[a] == self.r[b] {
            self.r[a] += 1;
        }
        true
    }
}

// edges: (weight, u, v)
fn max_spanning_tree(n: usize, mut edges: Vec<(i64, usize, usize)>) -> i64 {
    edges.sort_by(|a, b| b.0.cmp(&a.0));
    let mut dsu = Dsu::new(n);
    let mut total = 0;
    for (w, u, v) in edges {
        if dsu.unite(u, v) {
            total += w;
        }
    }
    total
}

fn main() {
    let edges = vec![(1, 0, 1), (2, 1, 2), (3, 2, 3), (4, 0, 3), (5, 0, 2)];
    println!("{}", max_spanning_tree(4, edges)); // 11
}
