// Day 865: Minimum cost to connect all houses to the plant = MST cost.
// Approach: Kruskal's algorithm with union-find over all pipe edges.
// Time: O(E log E), Space: O(V + E).
use std::collections::HashMap;

fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    let mut x = x;
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

fn main() {
    // (from, to, cost)
    let raw = [
        ("plant", "A", 1), ("plant", "B", 5), ("plant", "C", 20),
        ("A", "C", 15), ("B", "C", 10),
    ];
    let mut id: HashMap<&str, usize> = HashMap::new();
    let mut get_id = |id: &mut HashMap<&str, usize>, s: &'static str| -> usize {
        let n = id.len();
        *id.entry(s).or_insert(n)
    };

    let mut edges: Vec<(i32, usize, usize)> = Vec::new();
    for &(a, b, c) in &raw {
        let ua = get_id(&mut id, a);
        let ub = get_id(&mut id, b);
        edges.push((c, ua, ub));
    }
    edges.sort();

    let mut parent: Vec<usize> = (0..id.len()).collect();
    let mut total = 0;
    for &(c, u, v) in &edges {
        let ru = find(&mut parent, u);
        let rv = find(&mut parent, v);
        if ru != rv {
            parent[ru] = rv;
            total += c;
        }
    }
    println!("{}", total); // 16
}
