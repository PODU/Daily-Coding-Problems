// Balanced brackets via stack. Time O(n), Space O(n).
fn is_balanced(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => if stack.pop() != Some('(') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            _ => {}
        }
    }
    stack.is_empty()
}

fn main() {
    assert!(!is_balanced("([)]"));
    assert!(!is_balanced("((()"));
    println!("{}", if is_balanced("([])[]({})") { "true" } else { "false" });
}
