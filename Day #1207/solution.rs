// Day 1207: Remove kth-from-last node in one pass, constant space.
// Two pointers k apart; when lead hits end, trail marks the target. Time O(n), Space O(1).
// Modeled over a slice to keep ownership simple while preserving the one-pass two-pointer logic.
fn remove_kth_last(vals: &[i32], k: usize) -> Vec<i32> {
    let mut lead = k; // lead advanced k nodes ahead
    let mut trail = 0usize; // node to remove (0-based index = len - k)
    while lead < vals.len() {
        lead += 1;
        trail += 1;
    }
    // trail now equals len - k, the index of the kth-from-last node
    vals.iter().enumerate().filter(|&(i, _)| i != trail).map(|(_, &v)| v).collect()
}

fn main() {
    let list = vec![1, 2, 3, 4, 5];
    let res = remove_kth_last(&list, 2); // remove 4
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" -> ")); // 1 -> 2 -> 3 -> 5
}
