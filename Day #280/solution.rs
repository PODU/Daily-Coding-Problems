// Day 280: Detect cycle in undirected graph via Union-Find.
// An edge joining two already-connected vertices implies a cycle.
// Time O(V + E * alpha(V)), Space O(V).
fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    let mut x = x;
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

fn has_cycle(n: usize, edges: &[(usize, usize)]) -> bool {
    let mut parent: Vec<usize> = (0..n).collect();
    for &(u, v) in edges {
        let ru = find(&mut parent, u);
        let rv = find(&mut parent, v);
        if ru == rv {
            return true;
        }
        parent[ru] = rv;
    }
    false
}

fn main() {
    println!("{}", has_cycle(4, &[(0, 1), (1, 2), (2, 0), (2, 3)])); // true
    println!("{}", has_cycle(4, &[(0, 1), (1, 2), (2, 3)]));          // false
}
