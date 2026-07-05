// Day 1771: Stack-based "basic calculator" for +,-,parentheses,single digits,unary sign.
// Single left-to-right pass with a sign/result stack. Time: O(n), Space: O(n).

fn evaluate(s: &str) -> i64 {
    let mut result: i64 = 0;
    let mut sign: i64 = 1;
    let mut stack: Vec<(i64, i64)> = Vec::new(); // saved (result, sign) at '('
    for c in s.chars() {
        if c.is_ascii_digit() {
            result += sign * (c as i64 - '0' as i64);
            sign = 1;
        } else if c == '+' {
            sign = 1;
        } else if c == '-' {
            sign = -1;
        } else if c == '(' {
            stack.push((result, sign));
            result = 0;
            sign = 1;
        } else if c == ')' {
            let (prev_res, prev_sign) = stack.pop().unwrap();
            result = prev_res + prev_sign * result;
            sign = 1;
        }
    }
    result
}

fn main() {
    println!("{}", evaluate("-1 + (2 + 3)"));
}
