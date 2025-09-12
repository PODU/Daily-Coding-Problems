// Day 265: Minimum bonuses. Two-pass scan (left-to-right then right-to-left),
// each worker gets max of the two passes. Time O(n), space O(n).

fn bonuses(lines: &[i64]) -> Vec<i64> {
    let n = lines.len();
    let mut b = vec![1i64; n];
    for i in 1..n {
        if lines[i] > lines[i - 1] {
            b[i] = b[i - 1] + 1;
        }
    }
    for i in (0..n.saturating_sub(1)).rev() {
        if lines[i] > lines[i + 1] {
            b[i] = b[i].max(b[i + 1] + 1);
        }
    }
    b
}

fn main() {
    let lines = vec![10i64, 40, 200, 1000, 60, 30];
    let b = bonuses(&lines);
    let parts: Vec<String> = b.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
