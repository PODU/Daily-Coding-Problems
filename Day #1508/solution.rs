// Product of array except self without division: prefix-product then suffix-product.
// Two passes, output array only. Time O(n), Space O(1) extra (besides output).

fn product_except_self(nums: &[i64]) -> Vec<i64> {
    let n = nums.len();
    let mut res = vec![1i64; n];
    for i in 1..n {
        res[i] = res[i - 1] * nums[i - 1];
    }
    let mut suffix = 1i64;
    for i in (0..n).rev() {
        res[i] *= suffix;
        suffix *= nums[i];
    }
    res
}

fn main() {
    let nums: Vec<i64> = vec![1, 2, 3, 4, 5];
    let res = product_except_self(&nums);
    let parts: Vec<String> = res.iter().map(|x| x.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
