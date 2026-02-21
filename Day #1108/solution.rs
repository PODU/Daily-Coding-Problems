// Day 1108: Detect a cycle in an undirected graph using Union-Find.
// If an edge connects two already-connected vertices, there's a cycle.
// Time: O(E * alpha(V)). Space: O(V).
struct DSU {
    parent: Vec<usize>,
    rank: Vec<u32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU { parent: (0..n).collect(), rank: vec![0; n] }
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

fn has_cycle(n: usize, edges: &[(usize, usize)]) -> bool {
    let mut d = DSU::new(n);
    for &(a, b) in edges {
        if !d.unite(a, b) {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", has_cycle(3, &[(0, 1), (1, 2), (2, 0)])); // true
    println!("{}", has_cycle(4, &[(0, 1), (1, 2), (2, 3)])); // false
}
