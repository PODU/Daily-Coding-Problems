// Largest product of any three integers.
// Sort once; answer is max of top-3 product and (two smallest * largest).
// Time O(n log n), Space O(1).
fn largest_product_of_three(a: &[i64]) -> i64 {
    let mut v = a.to_vec();
    v.sort();
    let n = v.len();
    (v[n - 1] * v[n - 2] * v[n - 3]).max(v[0] * v[1] * v[n - 1])
}

fn main() {
    println!("{}", largest_product_of_three(&[-10, -10, 5, 2]));
}
