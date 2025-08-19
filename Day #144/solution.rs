// Nearest larger element's index by index-distance: expand outward from i. O(n) per query, O(1) space.

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
        Some(idx) => println!("{}", idx), // 3
        None => println!("null"),
    }
}
