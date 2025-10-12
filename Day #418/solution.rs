// Day 418: Two-pass greedy. Each gets >= 1; more than any lower neighbor. Like candy distribution.
// Time O(n), Space O(n).
fn bonuses(lines: &[i32]) -> Vec<i32> {
    let n = lines.len();
    let mut res = vec![1; n];
    for i in 1..n {
        if lines[i] > lines[i - 1] {
            res[i] = res[i - 1] + 1;
        }
    }
    for i in (0..n.saturating_sub(1)).rev() {
        if lines[i] > lines[i + 1] {
            res[i] = res[i].max(res[i + 1] + 1);
        }
    }
    res
}

fn main() {
    let res = bonuses(&[10, 40, 200, 1000, 60, 30]);
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
