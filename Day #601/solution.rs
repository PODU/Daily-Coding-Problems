// Day 601: Reconstruct a permutation of [0..N] matching +/- relations between neighbors.
// Approach: greedy low/high pointers (DI-match). Time O(n), Space O(n). Any consistent array is valid.

// signs[0] is the placeholder (None); signs[k] is '+' if a[k] > a[k-1], else '-'.
fn reconstruct(signs: &[char]) -> Vec<i32> {
    let n = signs.len() as i32; // numbers 0..n-1
    let (mut low, mut high) = (0i32, n - 1);
    let mut res = Vec::new();
    for k in 1..signs.len() {
        if signs[k] == '+' {
            res.push(low);
            low += 1;
        } else {
            res.push(high);
            high -= 1;
        }
    }
    res.push(low);
    res
}

fn main() {
    // [None, +, +, -, +]  (' ' stands in for None)
    let signs = [' ', '+', '+', '-', '+'];
    println!("{:?}", reconstruct(&signs));
}
