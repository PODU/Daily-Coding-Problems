// Day 362: Strobogrammatic numbers of N digits.
// Recursively build from outside in using rotatable digit pairs; skip leading 0.
// Time O(output size), Space O(N) recursion depth.

fn build(n: i32, total: i32) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }
    if n == 1 {
        return vec!["0".into(), "1".into(), "8".into()];
    }
    let pairs = [('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')];
    let inner = build(n - 2, total);
    let mut res = Vec::new();
    for s in &inner {
        for &(a, b) in &pairs {
            if n == total && a == '0' {
                continue;
            }
            res.push(format!("{}{}{}", a, s, b));
        }
    }
    res
}

fn strobogrammatic(n: i32) -> Vec<String> {
    build(n, n)
}

fn main() {
    let n = 2;
    println!("N={}: {}", n, strobogrammatic(n).join(" "));
}
