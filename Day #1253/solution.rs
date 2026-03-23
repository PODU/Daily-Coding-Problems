// Balanced brackets via stack: push openers, match closers to top. O(n) time, O(n) space.
fn is_balanced(s: &str) -> bool {
    let mut st: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => st.push(c),
            ')' => {
                if st.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if st.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if st.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    st.is_empty()
}

fn main() {
    println!("{}", is_balanced("([])[]({})"));
    println!("{}", is_balanced("([)]"));
}
