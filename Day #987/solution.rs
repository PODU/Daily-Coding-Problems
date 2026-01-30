// Day 987: Nearest larger number index.
// Expand outward from i checking i-d and i+d; first larger wins. O(n) per query, O(1) space.
// Follow-up: with O(n^2) preprocessing (or sparse tables) each query can be answered in O(1).

// Returns Some(index) of nearest larger element, or None if none.
fn nearest_larger(a: &[i64], i: usize) -> Option<usize> {
    let n = a.len();
    for d in 1..n {
        if i >= d && a[i - d] > a[i] {
            return Some(i - d); // prefer left on ties
        }
        let r = i + d;
        if r < n && a[r] > a[i] {
            return Some(r);
        }
    }
    None
}

fn main() {
    let a = [4, 1, 3, 5, 6];
    match nearest_larger(&a, 0) {
        Some(idx) => println!("{}", idx), // expected 3
        None => println!("null"),
    }
}
