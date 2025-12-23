// Largest product of three: one pass tracking 3 largest + 2 smallest.
// answer = max(max1*max2*max3, max1*min1*min2). Time O(n), Space O(1).

fn largest_product(nums: &[i64]) -> i64 {
    let mut max1 = i64::MIN;
    let mut max2 = i64::MIN;
    let mut max3 = i64::MIN;
    let mut min1 = i64::MAX;
    let mut min2 = i64::MAX;
    for &x in nums {
        if x > max1 {
            max3 = max2;
            max2 = max1;
            max1 = x;
        } else if x > max2 {
            max3 = max2;
            max2 = x;
        } else if x > max3 {
            max3 = x;
        }
        if x < min1 {
            min2 = min1;
            min1 = x;
        } else if x < min2 {
            min2 = x;
        }
    }
    (max1 * max2 * max3).max(max1 * min1 * min2)
}

fn main() {
    let nums = [-10i64, -10, 5, 2];
    println!("{}", largest_product(&nums));
}
