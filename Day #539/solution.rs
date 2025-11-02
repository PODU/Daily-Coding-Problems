// Day 539: Detect a cycle in an undirected graph using union-find (disjoint set).
// For each edge, if endpoints already share a root -> cycle. Time O(E*alpha(V)), Space O(V).

struct Dsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu { parent: (0..n).collect(), rank: vec![0; n] }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let r = self.find(self.parent[x]);
            self.parent[x] = r;
        }
        self.parent[x]
    }
    fn unite(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return false; // already connected -> cycle
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

fn has_cycle(n: usize, edges: &[(usize, usize)]) -> bool {
    let mut d = Dsu::new(n);
    for &(a, b) in edges {
        if !d.unite(a, b) {
            return true;
        }
    }
    false
}

fn main() {
    let cyclic = [(0, 1), (1, 2), (2, 3), (3, 0)];
    let tree = [(0, 1), (1, 2), (2, 3)];
    println!("{}", has_cycle(4, &cyclic));
    println!("{}", has_cycle(4, &tree));
}
