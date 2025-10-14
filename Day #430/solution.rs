// Day 430: Balance parentheses with the minimum number of insertions + deletions.
// One pass: keep matched pairs; unmatched ')' -> "()", leftover '(' gets a ')'. Time O(n), Space O(n).
fn balance(s: &str) -> String {
    let mut res = String::new();
    let mut open = 0;
    for c in s.chars() {
        if c == '(' {
            open += 1;
            res.push('(');
        } else {
            // ')'
            if open > 0 {
                open -= 1;
                res.push(')');
            } else {
                res.push_str("()");
            }
        }
    }
    for _ in 0..open {
        res.push(')');
    }
    res
}

fn main() {
    println!("{}", balance("(()"));
    println!("{}", balance("))()("));
}
