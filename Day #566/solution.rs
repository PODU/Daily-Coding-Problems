// Minimally-connected = tree: Union-Find detects cycle on union; final check connected && edges==V-1. Time O(E a(V)), Space O(V).
fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

fn is_minimally_connected(v: usize, edges: &[(usize, usize)]) -> bool {
    if edges.len() != v - 1 {
        return false;
    }
    let mut parent: Vec<usize> = (0..v).collect();
    for &(a, b) in edges {
        let ra = find(&mut parent, a);
        let rb = find(&mut parent, b);
        if ra == rb {
            return false; // cycle
        }
        parent[ra] = rb;
    }
    let root = find(&mut parent, 0);
    (1..v).all(|i| find(&mut parent, i) == root)
}

fn main() {
    println!("{}", if is_minimally_connected(4, &[(0, 1), (1, 2), (2, 3)]) { "True" } else { "False" });
    println!("{}", if is_minimally_connected(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]) { "True" } else { "False" });
}
