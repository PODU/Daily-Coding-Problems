// Day 86: Min parentheses to remove for validity. Track unmatched '(' and ')'.
// Time O(n), Space O(1).
fn min_removal(s: &str) -> i32 {
    let mut open = 0;
    let mut removals = 0;
    for c in s.chars() {
        match c {
            '(' => open += 1,
            ')' => {
                if open > 0 {
                    open -= 1;
                } else {
                    removals += 1; // unmatched ')'
                }
            }
            _ => {}
        }
    }
    removals + open // leftover unmatched '('
}

fn main() {
    println!("{}", min_removal("()())()")); // 1
    println!("{}", min_removal(")(")); // 2
}
