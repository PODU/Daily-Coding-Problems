// Product of array except self via prefix and suffix passes (no division).
// Time: O(n), Space: O(1) extra (excluding output).
fn product_except_self(nums: &[i64]) -> Vec<i64> {
    let n = nums.len();
    let mut res = vec![1i64; n];
    let mut pre = 1i64;
    for i in 0..n {
        res[i] = pre;
        pre *= nums[i];
    }
    let mut suf = 1i64;
    for i in (0..n).rev() {
        res[i] *= suf;
        suf *= nums[i];
    }
    res
}

fn main() {
    println!("{:?}", product_except_self(&[1, 2, 3, 4, 5]));
}
