// Largest product of three: track top-3 max and bottom-2 min in one pass.
// Answer = max(max1*max2*max3, min1*min2*max1). O(n) time, O(1) space.

fn largest_product_of_three(a: &[i64]) -> i64 {
    let (mut max1, mut max2, mut max3) = (i64::MIN, i64::MIN, i64::MIN);
    let (mut min1, mut min2) = (i64::MAX, i64::MAX);
    for &x in a {
        if x > max1 { max3 = max2; max2 = max1; max1 = x; }
        else if x > max2 { max3 = max2; max2 = x; }
        else if x > max3 { max3 = x; }
        if x < min1 { min2 = min1; min1 = x; }
        else if x < min2 { min2 = x; }
    }
    (max1 * max2 * max3).max(min1 * min2 * max1)
}

fn main() {
    println!("{}", largest_product_of_three(&[-10, -10, 5, 2])); // 500
}
