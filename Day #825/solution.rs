// Sorted squares via two-pointer merge from both ends, filling result from the back.
// Time: O(n), Space: O(n).

fn sorted_squares(nums: &[i64]) -> Vec<i64> {
    let n = nums.len();
    let mut res = vec![0i64; n];
    let mut lo = 0usize;
    let mut hi = n - 1;
    for k in (0..n).rev() {
        let l = nums[lo] * nums[lo];
        let r = nums[hi] * nums[hi];
        if l > r {
            res[k] = l;
            lo += 1;
        } else {
            res[k] = r;
            if hi > 0 { hi -= 1; }
        }
    }
    res
}

fn main() {
    let res = sorted_squares(&[-9, -2, 0, 2, 3]);
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
