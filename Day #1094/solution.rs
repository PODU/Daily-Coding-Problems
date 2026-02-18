// Uniformly shuffle a linked list via Fisher-Yates on node values. Time O(n), Space O(n).
// Space-over-time alternative: walk to a random remaining node and swap in place -> O(1) extra, O(n^2) time.
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn build(arr: &[i32]) -> Option<Box<Node>> {
    let mut head: Option<Box<Node>> = None;
    for &x in arr.iter().rev() {
        head = Some(Box::new(Node { val: x, next: head }));
    }
    head
}

fn to_vec(mut h: &Option<Box<Node>>) -> Vec<i32> {
    let mut v = Vec::new();
    while let Some(n) = h {
        v.push(n.val);
        h = &n.next;
    }
    v
}

fn main() {
    let mut head = build(&[1, 2, 3, 4, 5]);
    println!("Original: {:?}", to_vec(&head));

    let mut vals = to_vec(&head);
    let n = vals.len();
    // Deterministic LCG so the demo is reproducible.
    let mut seed: u64 = 42;
    let mut rand = |m: usize| -> usize {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345) & 0x7fffffff;
        (seed as usize) % m
    };
    for i in (1..n).rev() {
        let j = rand(i + 1);
        vals.swap(i, j);
    }
    // Write shuffled values back into the list nodes.
    let mut cur = head.as_mut();
    let mut idx = 0;
    while let Some(node) = cur {
        node.val = vals[idx];
        idx += 1;
        cur = node.next.as_mut();
    }
    println!("Shuffled: {:?}", to_vec(&head));
}
