// Day 1288: Balance parentheses with minimum insertions+deletions (insertion-only is minimal).
// Single scan: pair each ')' with an open, else insert '(' before it; close leftover opens. O(n).
fn balance(s: &str) -> String {
    let mut res = String::new();
    let mut open = 0;
    for ch in s.chars() {
        if ch == '(' {
            res.push('(');
            open += 1;
        } else if open > 0 {
            res.push(')');
            open -= 1;
        } else {
            res.push_str("()"); // insert matching '(' before unmatched ')'
        }
    }
    for _ in 0..open {
        res.push(')');
    }
    res
}

fn main() {
    println!("{}", balance("(()"));   // (())
    println!("{}", balance("))()(")); // ()()()()
}
