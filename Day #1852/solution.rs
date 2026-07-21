// Day 1852: Longest Increasing Subsequence (strict).
// Patience sorting: maintain tails[]; binary-search insertion point. O(n log n) time, O(n) space.

fn lis(a: &[i32]) -> usize {
    let mut tails: Vec<i32> = Vec::new();
    for &x in a {
        match tails.binary_search(&x) {
            Ok(_) => {}                       // x already present (strict): replace at found pos
            Err(i) => {
                if i == tails.len() {
                    tails.push(x);
                } else {
                    tails[i] = x;
                }
            }
        }
    }
    tails.len()
}

fn main() {
    println!("{}", lis(&[10, 9, 2, 5, 3, 7, 101, 18])); // 4
}
