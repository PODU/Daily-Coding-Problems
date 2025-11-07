// Product of array except self without division.
// Prefix * suffix products in two passes. Time O(N), Space O(1) extra (besides output).
fn product_except_self(a: &[i64]) -> Vec<i64> {
    let n = a.len();
    let mut res = vec![1i64; n];
    let mut prefix: i64 = 1;
    for i in 0..n {
        res[i] = prefix;
        prefix *= a[i];
    }
    let mut suffix: i64 = 1;
    for i in (0..n).rev() {
        res[i] *= suffix;
        suffix *= a[i];
    }
    res
}

fn fmt(v: &[i64]) -> String {
    let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

fn main() {
    println!("{}", fmt(&product_except_self(&[1, 2, 3, 4, 5])));
    println!("{}", fmt(&product_except_self(&[3, 2, 1])));
}
