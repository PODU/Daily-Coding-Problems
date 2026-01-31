// Day 995: Product of array except self, without division.
// Left pass stores prefix products; right pass multiplies by suffix products.
// O(n) time, O(1) extra space (besides output).
fn products(nums: &[i64]) -> Vec<i64> {
    let n = nums.len();
    let mut res = vec![1i64; n];
    let mut left = 1i64;
    for i in 0..n {
        res[i] = left;
        left *= nums[i];
    }
    let mut right = 1i64;
    for i in (0..n).rev() {
        res[i] *= right;
        right *= nums[i];
    }
    res
}

fn main() {
    println!("{:?}", products(&[1, 2, 3, 4, 5])); // [120, 60, 40, 30, 24]
    println!("{:?}", products(&[3, 2, 1]));        // [2, 3, 6]
}
