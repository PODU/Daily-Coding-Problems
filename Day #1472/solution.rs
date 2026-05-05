// Day 1472: Largest product of any three integers.
// Track 3 largest and 2 smallest in one pass; max of two candidate products.
// Time O(N), Space O(1).

fn max_product_three(nums: &[i64]) -> i64 {
    let (mut max1, mut max2, mut max3) = (i64::MIN, i64::MIN, i64::MIN);
    let (mut min1, mut min2) = (i64::MAX, i64::MAX);
    for &n in nums {
        if n > max1 {
            max3 = max2;
            max2 = max1;
            max1 = n;
        } else if n > max2 {
            max3 = max2;
            max2 = n;
        } else if n > max3 {
            max3 = n;
        }
        if n < min1 {
            min2 = min1;
            min1 = n;
        } else if n < min2 {
            min2 = n;
        }
    }
    (max1 * max2 * max3).max(max1 * min1 * min2)
}

fn main() {
    println!("{}", max_product_three(&[-10, -10, 5, 2])); // 500
}
