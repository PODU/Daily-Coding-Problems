// Day 942: Min parentheses to remove to make the string valid.
// One pass: count unmatched ')' immediately + leftover unmatched '('. O(n) time, O(1) space.
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
    println!("{}", min_removals("()())()")); // 1
    println!("{}", min_removals(")("));        // 2
}
