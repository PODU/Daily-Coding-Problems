// Day 1416: evaluate a +/-/parenthesized expression of single digits, no eval.
// Approach: single scan with a sign stack (Basic Calculator). O(n) time, O(n) space.

fn evaluate(s: &str) -> i64 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut result: i64 = 0;
    let mut sign: i64 = 1;
    let mut stack: Vec<i64> = vec![1];
    let mut i = 0;
    while i < n {
        let c = bytes[i];
        if c.is_ascii_digit() {
            let mut num: i64 = 0;
            while i < n && bytes[i].is_ascii_digit() {
                num = num * 10 + (bytes[i] - b'0') as i64;
                i += 1;
            }
            result += sign * stack[stack.len() - 1] * num;
            continue;
        } else if c == b'+' {
            sign = 1;
        } else if c == b'-' {
            sign = -1;
        } else if c == b'(' {
            stack.push(sign * stack[stack.len() - 1]);
            sign = 1;
        } else if c == b')' {
            stack.pop();
        }
        i += 1;
    }
    result
}

fn main() {
    println!("{}", evaluate("-1 + (2 + 3)")); // 4
}
