// Count connected components (friend groups) via Union-Find.
// Time O(V+E a(V)), Space O(V).
use std::collections::{HashMap, HashSet};

struct Dsu {
    p: HashMap<i32, i32>,
}

impl Dsu {
    fn find(&mut self, x: i32) -> i32 {
        let mut x = x;
        while self.p[&x] != x {
            let parent = self.p[&x];
            let grand = self.p[&parent];
            self.p.insert(x, grand);
            x = grand;
        }
        x
    }
    fn unite(&mut self, a: i32, b: i32) {
        let (ra, rb) = (self.find(a), self.find(b));
        self.p.insert(ra, rb);
    }
}

fn count_groups(adj: &HashMap<i32, Vec<i32>>) -> usize {
    let mut dsu = Dsu { p: HashMap::new() };
    for &k in adj.keys() {
        dsu.p.insert(k, k);
    }
    for (&u, nbrs) in adj {
        for &v in nbrs {
            dsu.unite(u, v);
        }
    }
    let mut roots = HashSet::new();
    let keys: Vec<i32> = adj.keys().cloned().collect();
    for k in keys {
        let r = dsu.find(k);
        roots.insert(r);
    }
    roots.len()
}

fn main() {
    let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
    adj.insert(0, vec![1, 2]);
    adj.insert(1, vec![0, 5]);
    adj.insert(2, vec![0]);
    adj.insert(3, vec![6]);
    adj.insert(4, vec![]);
    adj.insert(5, vec![1]);
    adj.insert(6, vec![3]);
    println!("{}", count_groups(&adj));
}
