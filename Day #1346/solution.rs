// Expand outward from index i, returning nearest j (by |j-i|) with a[j] > a[i]; None if none.
// Time: O(n) per query, Space: O(1).
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
        Some(j) => println!("{}", j),
        None => println!("None"),
    }
}
