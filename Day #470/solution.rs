// Nearest larger: expand outward from i, return first index with greater value.
// Time: O(n), Space: O(1).
fn nearest_larger(a: &[i32], i: usize) -> Option<usize> {
    let n = a.len();
    for d in 1..n {
        if i >= d && a[i - d] > a[i] {
            return Some(i - d);
        }
        if i + d < n && a[i + d] > a[i] {
            return Some(i + d);
        }
    }
    None
}

fn main() {
    let a = [4, 1, 3, 5, 6];
    match nearest_larger(&a, 0) {
        Some(r) => println!("{}", r),
        None => println!("null"),
    }
}
