// Day 1486: Partition a linked list around pivot k (stable).
// Approach: split values into (< k) and (>= k) preserving order, then concatenate. O(n) time.

fn partition(vals: &[i32], k: i32) -> Vec<i32> {
    let mut less: Vec<i32> = Vec::new();
    let mut ge: Vec<i32> = Vec::new();
    for &v in vals {
        if v < k {
            less.push(v);
        } else {
            ge.push(v);
        }
    }
    less.extend(ge);
    less
}

fn main() {
    let result = partition(&[5, 1, 8, 0, 3], 3);
    let parts: Vec<String> = result.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" -> "));
}
