// Balance parentheses with min insertions/deletions via insertion-based scan.
// Time O(n), Space O(n).
fn balance(s: &str) -> String {
    let mut result = String::new();
    let mut open = 0;
    for c in s.chars() {
        if c == '(' {
            result.push('(');
            open += 1;
        } else { // ')'
            if open == 0 {
                result.push('(');
                result.push(')');
            } else {
                result.push(')');
                open -= 1;
            }
        }
    }
    for _ in 0..open {
        result.push(')');
    }
    result
}

fn main() {
    println!("{}", balance("(()"));
    println!("{}", balance("))()("));
}
