// Day 765: Remove the kth-from-last node in one pass with two pointers.
// fast leads slow by k; when fast hits the end, slow precedes the target.
// Time: O(n), Space: O(1). Modeled with a Vec arena (next = index, NIL = none).
const NIL: usize = usize::MAX;

fn remove_kth_last(nodes: &[(i32, usize)], head: usize, k: usize) -> Vec<i32> {
    // Two-pointer over indices; returns the resulting value sequence.
    // dummy is represented by starting "before head".
    let next = |i: usize| if i == NIL { NIL } else { nodes[i].1 };

    // fast starts k steps ahead of a virtual dummy that points to head.
    let mut fast = head;
    for _ in 0..k - 1 {
        fast = next(fast);
    }
    // slow tracks the node before target; init slow = dummy (NIL meaning before head)
    let mut slow_prev: usize = NIL;
    let mut slow = head;
    while next(fast) != NIL {
        fast = next(fast);
        slow_prev = slow;
        slow = next(slow);
    }
    // slow is the target; slow_prev precedes it (NIL => removing head).
    let new_head = if slow_prev == NIL { next(slow) } else { head };

    // Walk and collect, skipping the target index `slow`.
    let mut out = Vec::new();
    let mut p = new_head;
    while p != NIL {
        if p != slow {
            out.push(nodes[p].0);
        }
        p = next(p);
    }
    out
}

fn main() {
    // values 1..=5 as a linked list inside a Vec
    let mut nodes: Vec<(i32, usize)> = Vec::new();
    for v in 1..=5 {
        nodes.push((v, NIL));
    }
    for i in 0..4 {
        nodes[i].1 = i + 1;
    }
    let head = 0;

    let before: Vec<String> = (0..nodes.len()).map(|i| nodes[i].0.to_string()).collect();
    println!("before: {}", before.join(" -> "));

    let after = remove_kth_last(&nodes, head, 2); // remove value 4
    let after_s: Vec<String> = after.iter().map(|v| v.to_string()).collect();
    println!("after:  {}", after_s.join(" -> ")); // 1 -> 2 -> 3 -> 5
}
