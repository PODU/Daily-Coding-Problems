// XOR all -> a^b; isolate a differing bit; partition into two groups and XOR each.
// Time O(n); Space O(1).
fn two_singles(nums: &[i32]) -> (i32, i32) {
    let mut x = 0;
    for &v in nums {
        x ^= v;
    }
    let bit = x & (-x); // lowest set bit where the two singles differ
    let (mut a, mut b) = (0, 0);
    for &v in nums {
        if v & bit != 0 {
            a ^= v;
        } else {
            b ^= v;
        }
    }
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    (a, b)
}

fn main() {
    let (a, b) = two_singles(&[2, 4, 6, 8, 10, 2, 6, 10]);
    println!("{} and {}", a, b); // 4 and 8
}
