// Evaluate +/- expression with parentheses using a sign stack (Basic Calculator).
// Single linear scan; parentheses handled by pushing the running result and sign.
// Time: O(n), Space: O(n).

fn evaluate(s: &str) -> i64 {
    let bytes = s.as_bytes();
    let mut result: i64 = 0;
    let mut sign: i64 = 1;
    let mut stack: Vec<i64> = Vec::new();
    let mut i = 0;
    let n = bytes.len();
    while i < n {
        let c = bytes[i];
        if c.is_ascii_digit() {
            let mut num: i64 = 0;
            while i < n && bytes[i].is_ascii_digit() {
                num = num * 10 + (bytes[i] - b'0') as i64;
                i += 1;
            }
            result += sign * num;
            continue;
        } else if c == b'+' {
            sign = 1;
        } else if c == b'-' {
            sign = -1;
        } else if c == b'(' {
            stack.push(result);
            stack.push(sign);
            result = 0;
            sign = 1;
        } else if c == b')' {
            let s2 = stack.pop().unwrap();
            let r2 = stack.pop().unwrap();
            result = r2 + s2 * result;
        }
        i += 1;
    }
    result
}

fn main() {
    println!("{}", evaluate("-1 + (2 + 3)")); // 4
}
