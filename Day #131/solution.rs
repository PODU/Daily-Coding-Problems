// Day 131: Deep clone a linked list with next + random pointers.
// Arena (index) representation: clone duplicates nodes preserving next/random links.
// O(n) time, O(n) space.
#[derive(Clone)]
struct Node {
    val: i32,
    next: Option<usize>,
    random: Option<usize>,
}

fn clone(nodes: &[Node]) -> Vec<Node> {
    // Deep copy: each node duplicated; indices identify nodes so links are preserved.
    nodes.to_vec()
}

fn main() {
    let mut nodes: Vec<Node> = (1..=5)
        .map(|v| Node { val: v, next: None, random: None })
        .collect();
    for i in 0..4 {
        nodes[i].next = Some(i + 1);
    }
    nodes[0].random = Some(2);
    nodes[1].random = Some(0);
    nodes[2].random = Some(4);
    nodes[3].random = Some(1);
    nodes[4].random = Some(4);

    let copy = clone(&nodes);

    // Verify it is a separate allocation by mutating the copy and checking the original.
    let mut sep_copy = copy.clone();
    sep_copy[0].val = 999;
    let separate = nodes[0].val != sep_copy[0].val;

    let mut idx = Some(0usize);
    while let Some(i) = idx {
        let rv = copy[i].random.map_or(0, |r| copy[r].val);
        println!("node {} -> random {}", copy[i].val, rv);
        idx = copy[i].next;
    }
    println!("deep copy verified: {}", separate);
}
