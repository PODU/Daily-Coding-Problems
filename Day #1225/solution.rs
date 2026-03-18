// Min parens to remove for validity: single pass counting unmatched.
// Time O(n), Space O(1).
fn min_removals(s: &str) -> i32 {
    let mut open = 0;
    let mut removals = 0;
    for c in s.chars() {
        if c == '(' {
            open += 1;
        } else if c == ')' {
            if open > 0 {
                open -= 1;
            } else {
                removals += 1;
            }
        }
    }
    removals + open
}

fn main() {
    println!("{}", min_removals("()())()"));
    println!("{}", min_removals(")("));
}
