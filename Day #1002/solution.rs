// Day 1002: Smallest positive integer not expressible as a subset sum (sorted array).
// If the next element x <= res (smallest unreachable, init 1) extend to res+x,
// else res is the answer. O(N) time, O(1) space.
fn smallest_non_subset_sum(nums: &[i64]) -> i64 {
    let mut res = 1i64;
    for &x in nums {
        if x > res {
            break;
        }
        res += x;
    }
    res
}

fn main() {
    println!("{}", smallest_non_subset_sum(&[1, 2, 3, 10])); // 7
}
