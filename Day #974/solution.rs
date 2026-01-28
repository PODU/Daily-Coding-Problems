// Day 974: Evaluate expression with parentheses, digits, +/- (no eval).
// Approach: single scan with a sign/result stack. Time O(n), Space O(n).

fn evaluate(s: &str) -> i64 {
    let mut result: i64 = 0;
    let mut num: i64 = 0;
    let mut sign: i64 = 1;
    let mut stack: Vec<i64> = Vec::new(); // alternating result, sign
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = num * 10 + (c as i64 - '0' as i64);
        } else if c == '+' {
            result += sign * num;
            num = 0;
            sign = 1;
        } else if c == '-' {
            result += sign * num;
            num = 0;
            sign = -1;
        } else if c == '(' {
            stack.push(result);
            stack.push(sign);
            result = 0;
            sign = 1;
        } else if c == ')' {
            result += sign * num;
            num = 0;
            let prev_sign = stack.pop().unwrap();
            let prev_result = stack.pop().unwrap();
            result = prev_result + prev_sign * result;
            sign = 1;
        }
    }
    result += sign * num;
    result
}

fn main() {
    println!("{}", evaluate("-1 + (2 + 3)")); // 4
}
