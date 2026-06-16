// Day 1675: Detect a cycle in an undirected graph via Union-Find.
// Union endpoints; a cycle exists if an edge joins already-connected nodes.
// Time O(E*alpha(V)), Space O(V).
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
    for &(a, b) in edges {
        let ra = find(&mut parent, a);
        let rb = find(&mut parent, b);
        if ra == rb {
            return true;
        }
        parent[ra] = rb;
    }
    false
}

fn main() {
    println!("{}", has_cycle(4, &[(0, 1), (1, 2), (2, 3), (3, 0)])); // true
    println!("{}", has_cycle(4, &[(0, 1), (1, 2), (2, 3)]));         // false
}
