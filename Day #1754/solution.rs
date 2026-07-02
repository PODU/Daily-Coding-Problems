// Day 1754: All strobogrammatic numbers with N digits.
// Build from middle outward placing rotatable pairs; skip leading 0 for outer layer.
// Time O(output size), space O(N) recursion depth.

const PAIRS: [(char, char); 5] = [('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')];

fn build(n: i32, total: i32) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }
    if n == 1 {
        return vec!["0".into(), "1".into(), "8".into()];
    }
    let inner = build(n - 2, total);
    let mut res = Vec::new();
    for s in &inner {
        for &(a, b) in PAIRS.iter() {
            if a == '0' && n == total {
                continue; // no leading zero
            }
            let mut t = String::new();
            t.push(a);
            t.push_str(s);
            t.push(b);
            res.push(t);
        }
    }
    res
}

fn strobogrammatic(n: i32) -> Vec<String> {
    build(n, n)
}

fn main() {
    for n in [2, 3] {
        println!("N={}: [{}]", n, strobogrammatic(n).join(", "));
    }
}
