// Day 1110: Detect a Pythagorean triplet a^2+b^2=c^2 in an array.
// Square all, sort; for each largest c use two-pointer scan. Time: O(N^2). Space: O(N).
fn has_triplet(arr: &[i32]) -> bool {
    let mut sq: Vec<i32> = arr.iter().map(|x| x * x).collect();
    sq.sort();
    let n = sq.len();
    if n < 3 {
        return false;
    }
    for c in (2..n).rev() {
        let (mut l, mut r) = (0usize, c - 1);
        while l < r {
            let s = sq[l] + sq[r];
            if s == sq[c] {
                return true;
            }
            if s < sq[c] {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
    false
}

fn main() {
    println!("{}", has_triplet(&[3, 1, 4, 6, 5]));   // true (3,4,5)
    println!("{}", has_triplet(&[10, 4, 6, 12, 5])); // false
}
