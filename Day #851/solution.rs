// Day 851: a graph is minimally-connected iff it is a tree: connected AND edges == nodes-1.
// Union-Find detects cycles and connectivity in O(E alpha(V)).
fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    let mut x = x;
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

fn is_minimally_connected(n: usize, edges: &[(usize, usize)]) -> bool {
    if edges.len() != n - 1 {
        return false;
    }
    let mut parent: Vec<usize> = (0..n).collect();
    for &(a, b) in edges {
        let ra = find(&mut parent, a);
        let rb = find(&mut parent, b);
        if ra == rb {
            return false; // cycle
        }
        parent[ra] = rb;
    }
    let mut roots = std::collections::HashSet::new();
    for i in 0..n {
        let r = find(&mut parent, i);
        roots.insert(r);
    }
    roots.len() == 1
}

fn main() {
    println!("{}", is_minimally_connected(5, &[(0, 1), (0, 2), (1, 3), (1, 4)])); // true
    println!("{}", is_minimally_connected(4, &[(0, 1), (1, 2), (2, 0), (2, 3)])); // false
}
