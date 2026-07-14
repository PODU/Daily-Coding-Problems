// Longest strictly increasing subsequence via patience sorting + binary search.
// Time: O(n log n). Space: O(n).
fn lis(a: &[i32]) -> usize {
    let mut tails: Vec<i32> = Vec::new();
    for &x in a {
        match tails.binary_search(&x) {
            Ok(_) => {} // x already present (strict): position found, value equal -> skip update
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
    let a = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];
    println!("{}", lis(&a)); // 6
}
