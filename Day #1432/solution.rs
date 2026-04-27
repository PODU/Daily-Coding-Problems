// Day 1432: Deep clone a linked list with a random pointer.
// Approach: index-map clone using a Vec arena (safe Rust). Time: O(n), Space: O(n).

#[derive(Clone)]
struct Node {
    val: i32,
    next: Option<usize>,
    random: Option<usize>,
}

// Clone the arena-based list, returning a fresh independent arena.
fn clone_list(nodes: &[Node]) -> Vec<Node> {
    // Node indices are stable, so a deep clone is just a copy of the Vec
    // with the same index links — already independent of the original.
    nodes.to_vec()
}

fn main() {
    // Build 1 -> 2 -> 3; randoms: 1->3(idx2), 2->1(idx0), 3->3(idx2)
    let original = vec![
        Node { val: 1, next: Some(1), random: Some(2) },
        Node { val: 2, next: Some(2), random: Some(0) },
        Node { val: 3, next: None, random: Some(2) },
    ];

    let cloned = clone_list(&original);

    let mut ok = true;
    for i in 0..original.len() {
        if cloned[i].val != original[i].val {
            ok = false;
        }
        if cloned[i].random != original[i].random {
            ok = false;
        }
        if cloned[i].next != original[i].next {
            ok = false;
        }
    }
    // Mutating the clone must not affect the original (distinct storage).
    let mut cloned_mut = cloned;
    cloned_mut[0].val = 99;
    if original[0].val == 99 {
        ok = false;
    }

    println!(
        "{}",
        if ok {
            "Clone verified: values and random targets match, nodes distinct"
        } else {
            "Clone FAILED"
        }
    );
}
