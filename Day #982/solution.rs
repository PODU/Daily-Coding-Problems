// Strobogrammatic numbers of N digits: build recursively from outside in using rotatable
// pairs {0-0,1-1,8-8,6-9,9-6}; skip leading 0 for N>1. Time O(5^(N/2)), Space O(N).
const PAIRS: [(char, char); 5] = [('0', '0'), ('1', '1'), ('8', '8'), ('6', '9'), ('9', '6')];

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
            res.push(format!("{}{}{}", a, s, b));
        }
    }
    res
}

fn strobogrammatic(n: i32) -> Vec<String> {
    build(n, n)
}

fn main() {
    println!("N=2: {:?}", strobogrammatic(2));
    println!("N=1: {:?}", strobogrammatic(1));
}
