// Intersection: two pointers switch lists after end; meet at the join. O(M+N) time, O(1) space.
// Lists are modeled as sequences of node ids; shared nodes carry equal ids.

fn get_intersection(a: &[usize], b: &[usize]) -> Option<usize> {
    if a.is_empty() || b.is_empty() {
        return None;
    }
    // Two cursors; each restarts on the other list at its end so both travel M+N steps.
    let mut i = 0usize; // index into the conceptual a-then-b stream
    let mut j = 0usize; // index into the conceptual b-then-a stream
    let len = a.len() + b.len();
    let node_a = |k: usize| if k < a.len() { Some(a[k]) } else { b.get(k - a.len()).copied() };
    let node_b = |k: usize| if k < b.len() { Some(b[k]) } else { a.get(k - b.len()).copied() };
    while i < len && j < len {
        let pa = node_a(i);
        let pb = node_b(j);
        if pa == pb {
            return pa;
        }
        i += 1;
        j += 1;
    }
    None
}

fn main() {
    // ids: A = 3(0) 7(1) [8(2) 10(3)] ; B = 99(4) 1(5) [8(2) 10(3)]
    let a = [0usize, 1, 2, 3];
    let b = [4usize, 5, 2, 3];
    let vals = [3, 7, 8, 10, 99, 1];
    let join = get_intersection(&a, &b).unwrap();
    println!("The node with value {}", vals[join]);
}
