// XOR all -> a^b; partition by a set bit, XOR each group to recover the two singletons. O(n) time, O(1) space.

fn main() {
    let nums = [2, 4, 6, 8, 10, 2, 6, 10];
    let mut xor_all = 0i32;
    for &x in &nums {
        xor_all ^= x;
    }
    let bit = xor_all & (-xor_all); // lowest set bit
    let (mut a, mut b) = (0i32, 0i32);
    for &x in &nums {
        if x & bit != 0 {
            a ^= x;
        } else {
            b ^= x;
        }
    }
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    println!("{} and {}", a, b);
}
