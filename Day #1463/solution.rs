// Pythagorean triplet: square all, sort, for each largest c^2 two-pointer for a^2+b^2.
// Time: O(n^2), Space: O(n).
fn has_triplet(input: &[i64]) -> bool {
    let mut arr: Vec<i64> = input.iter().map(|&x| x * x).collect();
    arr.sort();
    let n = arr.len();
    for c in (2..n).rev() {
        let (mut l, mut r) = (0usize, c - 1);
        while l < r {
            let s = arr[l] + arr[r];
            if s == arr[c] {
                return true;
            } else if s < arr[c] {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
    false
}

fn main() {
    println!("{}", if has_triplet(&[3, 1, 4, 6, 5]) { "True" } else { "False" });
    println!("{}", if has_triplet(&[10, 4, 6, 12, 5]) { "True" } else { "False" });
}
