// Day 1681: Min parentheses to remove for validity. Track unmatched '(' and count
// unmatched ')'; answer = leftover open + unmatched close. Time O(n), Space O(1).
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
    println!("{}", min_removals(")("));       // 2
}
