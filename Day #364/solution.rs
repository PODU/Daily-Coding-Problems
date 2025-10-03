// Day 364: Longest strictly increasing subsequence length.
// Patience sorting: keep tails[], binary-search first tail >= x and replace.
// Time O(n log n), Space O(n).

fn lis(a: &[i32]) -> usize {
    let mut tails: Vec<i32> = Vec::new();
    for &x in a {
        match tails.binary_search(&x) {
            Ok(_) => {} // x already present (strictly increasing): keep tails as is
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
