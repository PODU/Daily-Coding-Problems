// Detect a Pythagorean triplet a^2+b^2=c^2 in an array.
// Square + sort, then fix largest as c and two-pointer. Time O(n^2), Space O(1).

fn has_triplet(nums: &[i64]) -> bool {
    let mut sq: Vec<i64> = nums.iter().map(|x| x * x).collect();
    sq.sort();
    let n = sq.len();
    for c in (2..n).rev() {
        let (mut a, mut b) = (0usize, c - 1);
        while a < b {
            let s = sq[a] + sq[b];
            if s == sq[c] {
                return true;
            }
            if s < sq[c] {
                a += 1;
            } else {
                b -= 1;
            }
        }
    }
    false
}

fn main() {
    let nums: Vec<i64> = vec![3, 1, 4, 6, 5];
    println!("{}", has_triplet(&nums)); // expected: true
}
