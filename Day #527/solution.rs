// Day 527: Count distinct max-heaps from N distinct integers.
// Recurrence f(n) = C(n-1, L) * f(L) * f(R), L = left-subtree size of a complete
// binary tree with n nodes. u128 suffices for the demo. Time O(n^2), space O(n).

// number of nodes in the left subtree of a complete binary tree with n nodes
fn left_subtree_size(n: u64) -> u64 {
    if n <= 1 {
        return 0;
    }
    let mut h = 0u64;
    while (1u64 << (h + 1)) - 1 <= n {
        h += 1; // h = height (root at height 0)
    }
    let last_level_cap = 1u64 << h;
    let nodes_above = (1u64 << h) - 1;
    let last_level_nodes = n - nodes_above;
    let left_base = (1u64 << (h - 1)) - 1;
    let left_last = last_level_nodes.min(last_level_cap / 2);
    left_base + left_last
}

fn binom(n: u64, k: u64) -> u128 {
    let mut r: u128 = 1;
    for i in 0..k {
        r = r * (n - i) as u128 / (i + 1) as u128;
    }
    r
}

fn count_heaps(n: u64) -> u128 {
    if n <= 1 {
        return 1;
    }
    let l = left_subtree_size(n);
    let r = n - 1 - l;
    binom(n - 1, l) * count_heaps(l) * count_heaps(r)
}

fn main() {
    let n: u64 = 3;
    let _integers = [1, 2, 3];
    println!("{}", count_heaps(n)); // expected: 2
}
