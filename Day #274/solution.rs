// Day 274: Evaluate string of (), single digits, +/- without eval.
// Stack-based sign tracking. Time O(N), Space O(N).
fn evaluate(s: &str) -> i64 {
    let mut result: i64 = 0;
    let mut sign: i64 = 1;
    let mut st: Vec<i64> = Vec::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            result += sign * (c as i64 - '0' as i64);
        } else if c == '+' {
            sign = 1;
        } else if c == '-' {
            sign = -1;
        } else if c == '(' {
            st.push(result);
            st.push(sign);
            result = 0;
            sign = 1;
        } else if c == ')' {
            let s2 = st.pop().unwrap();
            let prev = st.pop().unwrap();
            result = prev + s2 * result;
        }
    }
    result
}

fn main() {
    println!("{}", evaluate("-1 + (2 + 3)")); // 4
}
