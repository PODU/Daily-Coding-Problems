// Day 809: Check balanced round/curly/square brackets using a stack.
// Push opens, match closes against stack top. Time O(N), Space O(N).

fn is_balanced(s: &str) -> bool {
    let mut stack = Vec::new();
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
    println!("{}", is_balanced("([])[]({})")); // true
    println!("{}", is_balanced("([)]"));        // false
    println!("{}", is_balanced("((()"));        // false
}
