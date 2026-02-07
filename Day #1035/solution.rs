// Day 1035: Smallest bonuses so each employee beats any lower-output neighbor.
// Two-pass greedy: left-to-right then right-to-left taking max. Time O(n), Space O(n).
fn bonuses(a: &[i64]) -> Vec<i64> {
    let n = a.len();
    let mut b = vec![1i64; n];
    for i in 1..n {
        if a[i] > a[i - 1] {
            b[i] = b[i - 1] + 1;
        }
    }
    for i in (0..n.saturating_sub(1)).rev() {
        if a[i] > a[i + 1] {
            b[i] = b[i].max(b[i + 1] + 1);
        }
    }
    b
}

fn main() {
    let a = [10i64, 40, 200, 1000, 60, 30];
    let b = bonuses(&a);
    let parts: Vec<String> = b.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
