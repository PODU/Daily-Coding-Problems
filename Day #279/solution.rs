// Day 279: Count friend groups = connected components via Union-Find.
// Time O(V + E * alpha(V)), Space O(V).
use std::collections::{HashMap, HashSet};

fn find(parent: &mut HashMap<i32, i32>, x: i32) -> i32 {
    let mut x = x;
    while parent[&x] != x {
        let gp = parent[&parent[&x]];
        parent.insert(x, gp);
        x = gp;
    }
    x
}

fn union(parent: &mut HashMap<i32, i32>, a: i32, b: i32) {
    let ra = find(parent, a);
    let rb = find(parent, b);
    parent.insert(ra, rb);
}

fn count_groups(adj: &HashMap<i32, Vec<i32>>) -> usize {
    let mut parent: HashMap<i32, i32> = adj.keys().map(|&u| (u, u)).collect();
    for (&u, nbrs) in adj {
        for &v in nbrs {
            union(&mut parent, u, v);
        }
    }
    let keys: Vec<i32> = adj.keys().cloned().collect();
    let mut roots = HashSet::new();
    for u in keys {
        let r = find(&mut parent, u);
        roots.insert(r);
    }
    roots.len()
}

fn main() {
    let adj: HashMap<i32, Vec<i32>> = [
        (0, vec![1, 2]), (1, vec![0, 5]), (2, vec![0]), (3, vec![6]),
        (4, vec![]), (5, vec![1]), (6, vec![3]),
    ]
    .into_iter()
    .collect();
    println!("{}", count_groups(&adj)); // 3
}
