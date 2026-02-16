// XOR all -> a^b; isolate via lowest set bit; partition & XOR each group to recover a,b; O(n) time O(1) space

fn find_two(nums: &[i32]) -> (i32, i32) {
    let xor_all = nums.iter().fold(0_i32, |acc, &n| acc ^ n);
    let bit = xor_all & (-xor_all); // lowest set bit that differs between a and b
    let (mut a, mut b) = (0_i32, 0_i32);
    for &n in nums {
        if n & bit != 0 {
            a ^= n;
        } else {
            b ^= n;
        }
    }
    (a.min(b), a.max(b))
}

fn main() {
    let nums = vec![2, 4, 6, 8, 10, 2, 6, 10];
    let (a, b) = find_two(&nums);
    println!("{} and {}", a, b);
}
