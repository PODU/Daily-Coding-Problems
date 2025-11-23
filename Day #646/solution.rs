// Day 646: Count friend groups = connected components in an undirected graph.
// Approach: Union-Find (DSU) with path compression + union by rank.
// Time: O(V + E * alpha(V)), Space: O(V).
use std::collections::{BTreeMap, HashSet};

struct Dsu { parent: Vec<usize>, rank: Vec<usize> }
impl Dsu {
    fn new(n: usize) -> Self { Dsu { parent: (0..n).collect(), rank: vec![0; n] } }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x { self.parent[x] = self.find(self.parent[x]); }
        self.parent[x]
    }
    fn unite(&mut self, a: usize, b: usize) {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b { return; }
        if self.rank[a] < self.rank[b] { std::mem::swap(&mut a, &mut b); }
        self.parent[b] = a;
        if self.rank[a] == self.rank[b] { self.rank[a] += 1; }
    }
}

fn count_groups(adj: &BTreeMap<usize, Vec<usize>>) -> usize {
    let mut dsu = Dsu::new(adj.len());
    for (&u, nbrs) in adj { for &v in nbrs { dsu.unite(u, v); } }
    let mut roots = HashSet::new();
    let keys: Vec<usize> = adj.keys().cloned().collect();
    for u in keys { roots.insert(dsu.find(u)); }
    roots.len()
}

fn main() {
    let mut adj: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    adj.insert(0, vec![1, 2]);
    adj.insert(1, vec![0, 5]);
    adj.insert(2, vec![0]);
    adj.insert(3, vec![6]);
    adj.insert(4, vec![]);
    adj.insert(5, vec![1]);
    adj.insert(6, vec![3]);
    println!("{}", count_groups(&adj)); // 3
}
