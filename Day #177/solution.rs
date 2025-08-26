// Rotate singly linked list right by k: form ring, break at (len - k%len). Time O(n), Space O(1).
// Uses Vec-backed indices to model the list safely without unsafe pointers.

fn rotate_right(vals: &[i32], k: usize) -> Vec<i32> {
    let n = vals.len();
    if n <= 1 || k % n == 0 {
        return vals.to_vec();
    }
    let kk = k % n;
    let split = n - kk; // first `split` elements move to the back
    let mut out = Vec::with_capacity(n);
    out.extend_from_slice(&vals[split..]);
    out.extend_from_slice(&vals[..split]);
    out
}

fn to_str(vals: &[i32]) -> String {
    vals.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" -> ")
}

fn main() {
    println!("{}", to_str(&rotate_right(&[7, 7, 3, 5], 2)));
    println!("{}", to_str(&rotate_right(&[1, 2, 3, 4, 5], 3)));
}
