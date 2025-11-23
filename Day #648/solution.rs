// Deep clone linked list with random pointers. Rust uses index-based arena (Vec) with
// Option<usize> links to avoid unsafe aliasing. Time O(n), Space O(n). fn main.

struct Node {
    val: i32,
    next: Option<usize>,
    random: Option<usize>,
}

// Clone the arena: since nodes are indexed, a deep clone is simply duplicating the Vec
// while preserving the same index-based links into the new arena.
fn clone_list(orig: &Vec<Node>) -> Vec<Node> {
    orig.iter()
        .map(|n| Node { val: n.val, next: n.next, random: n.random })
        .collect()
}

fn main() {
    // Build list 1->2->3->4 (indices 0..3)
    let mut nodes: Vec<Node> = vec![
        Node { val: 1, next: Some(1), random: Some(2) }, // node1.random=node3
        Node { val: 2, next: Some(2), random: Some(0) }, // node2.random=node1
        Node { val: 3, next: Some(3), random: Some(2) }, // node3.random=node3
        Node { val: 4, next: None,    random: Some(1) }, // node4.random=node2
    ];

    let cloned = clone_list(&nodes);

    // Verify it is a deep copy: mutate original, cloned must be unaffected.
    let orig_first_val = nodes[0].val;
    nodes[0].val = 999;
    let deep = cloned[0].val == orig_first_val && cloned.len() == nodes.len();

    // Print cloned list following next pointers from head (index 0).
    let mut idx = Some(0usize);
    while let Some(i) = idx {
        let n = &cloned[i];
        let rv = match n.random {
            Some(r) => cloned[r].val,
            None => 0,
        };
        println!("node {} random {}", n.val, rv);
        idx = n.next;
    }
    println!("deep copy verified: {}", deep);
}
