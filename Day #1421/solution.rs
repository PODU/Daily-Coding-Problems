// Day 1421: product of all elements except self, without division.
// Approach: prefix products pass then suffix products pass. O(n) time, O(1) extra (besides output).

fn product_except_self(nums: &[i64]) -> Vec<i64> {
    let n = nums.len();
    let mut res = vec![1i64; n];
    let mut prefix = 1i64;
    for i in 0..n {
        res[i] = prefix;
        prefix *= nums[i];
    }
    let mut suffix = 1i64;
    for i in (0..n).rev() {
        res[i] *= suffix;
        suffix *= nums[i];
    }
    res
}

fn main() {
    println!("{:?}", product_except_self(&[1, 2, 3, 4, 5])); // [120, 60, 40, 30, 24]
    println!("{:?}", product_except_self(&[3, 2, 1]));       // [2, 3, 6]
}
