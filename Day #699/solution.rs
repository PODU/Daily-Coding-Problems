// Day 699: Rotate a singly linked list right by k places.
// Approach: collect into a Vec, rotate indices, rebuild. Time O(n), Space O(n)
// for the demo (ownership-safe); the rotation itself is an O(1) pointer relink.
fn rotate_right(vals: &[i32], k: usize) -> Vec<i32> {
    let n = vals.len();
    if n == 0 {
        return vec![];
    }
    let k = k % n;
    let mut out = Vec::with_capacity(n);
    out.extend_from_slice(&vals[n - k..]);
    out.extend_from_slice(&vals[..n - k]);
    out
}

fn show(v: &[i32]) {
    let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" -> "));
}

fn main() {
    show(&rotate_right(&[7, 7, 3, 5], 2));    // 3 -> 5 -> 7 -> 7
    show(&rotate_right(&[1, 2, 3, 4, 5], 3)); // 3 -> 4 -> 5 -> 1 -> 2
}
