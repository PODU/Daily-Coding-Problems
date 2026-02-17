// Count connected components via Union-Find with path compression. Time ~O(N+E*alpha), Space O(N).
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

fn unite(parent: &mut HashMap<i32, i32>, a: i32, b: i32) {
    let ra = find(parent, a);
    let rb = find(parent, b);
    parent.insert(ra, rb);
}

fn main() {
    let adj: Vec<(i32, Vec<i32>)> = vec![
        (0, vec![1, 2]),
        (1, vec![0, 5]),
        (2, vec![0]),
        (3, vec![6]),
        (4, vec![]),
        (5, vec![1]),
        (6, vec![3]),
    ];
    let mut parent: HashMap<i32, i32> = HashMap::new();
    for (u, _) in &adj {
        parent.insert(*u, *u);
    }
    for (u, nbrs) in &adj {
        for v in nbrs {
            unite(&mut parent, *u, *v);
        }
    }
    let mut roots = HashSet::new();
    for (u, _) in &adj {
        let r = find(&mut parent, *u);
        roots.insert(r);
    }
    println!("{}", roots.len());
}
