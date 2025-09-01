// Day 199: Balance parentheses with minimum insertions/deletions.
// Greedy: each unmatched paren costs exactly 1 op; inserting its partner is always optimal.
// Time: O(n), Space: O(n).
fn balance(s: &str) -> String {
    let mut res = String::new();
    let mut open = 0;
    for c in s.chars() {
        if c == '(' {
            res.push('(');
            open += 1;
        } else if open > 0 {
            res.push(')');
            open -= 1;
        } else {
            res.push_str("()"); // unmatched ')': insert a '(' before it
        }
    }
    for _ in 0..open {
        res.push(')'); // close remaining opens
    }
    res
}

fn main() {
    println!("{}", balance("(()"));   // (())
    println!("{}", balance("))()(")); // ()()()()
}
