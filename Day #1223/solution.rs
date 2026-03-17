// Single linear scan; depth = max paren nesting ('(' +1, ')' -1).
// O(n) time, O(1) space.
fn tree_depth(s: &str) -> i32 {
    let mut depth = 0;
    let mut cur = 0;
    for c in s.chars() {
        match c {
            '(' => {
                cur += 1;
                if cur > depth {
                    depth = cur;
                }
            }
            ')' => cur -= 1,
            _ => {}
        }
    }
    depth
}

fn main() {
    println!("{}", tree_depth("((((00)0)0)0)"));
}
