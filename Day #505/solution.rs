// Day 505: Rotate array right by k in-place via three reversals.
// Time O(n), Space O(1).

fn reverse(a: &mut [i64], mut lo: usize, mut hi: usize) {
    while lo < hi {
        a.swap(lo, hi);
        lo += 1;
        hi -= 1;
    }
}

fn rotate_right(a: &mut Vec<i64>, k: usize) {
    let n = a.len();
    if n == 0 {
        return;
    }
    let k = k % n;
    reverse(a, 0, n - 1);
    if k > 0 {
        reverse(a, 0, k - 1);
    }
    reverse(a, k, n - 1);
}

fn format(a: &[i64]) -> String {
    let parts: Vec<String> = a.iter().map(|v| v.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

fn main() {
    let mut a = vec![1, 2, 3, 4, 5, 6, 7];
    rotate_right(&mut a, 3);
    println!("{}", format(&a)); // [5, 6, 7, 1, 2, 3, 4]
}
