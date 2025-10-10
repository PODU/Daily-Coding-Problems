// Strobogrammatic numbers of N digits: recursive build outside-in, skip leading 0 pair.
// Time O(5^(N/2)) results, Space O(N) recursion depth.
fn build(n: i32, total: i32) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }
    if n == 1 {
        return vec!["0".to_string(), "1".to_string(), "8".to_string()];
    }
    let pairs = [('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')];
    let inner = build(n - 2, total);
    let mut res = Vec::new();
    for s in &inner {
        for &(a, b) in pairs.iter() {
            if n == total && a == '0' {
                continue; // no leading zero
            }
            res.push(format!("{}{}{}", a, s, b));
        }
    }
    res
}

fn main() {
    let res = build(2, 2);
    let quoted: Vec<String> = res.iter().map(|x| format!("\"{}\"", x)).collect();
    println!("[{}]", quoted.join(", "));
}
