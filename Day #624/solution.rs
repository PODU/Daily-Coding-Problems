// Minimum parentheses to remove to make string valid: single pass counting unmatched
// open/close. Answer = removals (unmatched ')') + leftover open. Time O(n), Space O(1).

fn min_remove(s: &str) -> i32 {
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
    println!("{}", min_remove("()())()")); // 1
    println!("{}", min_remove(")("));       // 2
}
