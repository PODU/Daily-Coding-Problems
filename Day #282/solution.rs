// Day 282: Detect Pythagorean triplet. Square + sort, then two-pointer per c.
// Time O(N^2), Space O(N) for squares.
fn has_triplet(arr: &[i64]) -> bool {
    let mut a: Vec<i64> = arr.iter().map(|x| x * x).collect();
    a.sort();
    let n = a.len();
    for c in (2..n).rev() {
        let (mut lo, mut hi) = (0usize, c - 1);
        while lo < hi {
            let s = a[lo] + a[hi];
            if s == a[c] {
                return true;
            } else if s < a[c] {
                lo += 1;
            } else {
                hi -= 1;
            }
        }
    }
    false
}

fn main() {
    println!("{}", has_triplet(&[3, 1, 4, 6, 5]));    // true (3,4,5)
    println!("{}", has_triplet(&[10, 4, 6, 12, 5]));  // false
}
