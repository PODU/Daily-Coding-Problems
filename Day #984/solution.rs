// Balance a parentheses string with minimum insertions+deletions (insertion-only
// greedy is provably optimal: each unmatched paren needs exactly one edit).
// Time: O(n), Space: O(n).

fn balance(s: &str) -> String {
    let mut res = String::new();
    let mut bal = 0i32;
    for c in s.chars() {
        if c == '(' {
            res.push('(');
            bal += 1;
        } else {
            // ')'
            if bal > 0 {
                res.push(')');
                bal -= 1;
            } else {
                res.push_str("()"); // insert '(' to match this ')'
            }
        }
    }
    for _ in 0..bal {
        res.push(')'); // close any still-open '('
    }
    res
}

fn main() {
    println!("{}", balance("(()"));
    println!("{}", balance("))()("));
}
