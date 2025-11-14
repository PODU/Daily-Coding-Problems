// Day 597: Detect a Pythagorean triplet a^2 + b^2 = c^2 in an array.
// Approach: square values, sort, fix c as the largest, two-pointer. Time O(n^2), Space O(n).

fn has_pythagorean_triplet(nums: &[i64]) -> bool {
    let mut sq: Vec<i64> = nums.iter().map(|&v| v * v).collect();
    sq.sort();
    let n = sq.len();
    if n < 3 {
        return false;
    }
    for c in (2..n).rev() {
        let (mut lo, mut hi) = (0usize, c - 1);
        while lo < hi {
            let s = sq[lo] + sq[hi];
            if s == sq[c] {
                return true;
            } else if s < sq[c] {
                lo += 1;
            } else {
                hi -= 1;
            }
        }
    }
    false
}

fn main() {
    let arr = [3i64, 1, 4, 6, 5]; // contains 3,4,5
    println!("{}", has_pythagorean_triplet(&arr));
}
