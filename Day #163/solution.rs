// Day 163: Evaluate Reverse Polish Notation with a stack. Push operands, on an
// operator pop two and apply. Time O(n), Space O(n).

fn eval_rpn(tokens: &[&str]) -> i64 {
    let mut st: Vec<i64> = Vec::new();
    for &t in tokens {
        match t {
            "+" | "-" | "*" | "/" => {
                let b = st.pop().unwrap();
                let a = st.pop().unwrap();
                let r = match t {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    _ => a / b,
                };
                st.push(r);
            }
            num => st.push(num.parse().unwrap()),
        }
    }
    st.pop().unwrap()
}

fn main() {
    let tokens = ["15", "7", "1", "1", "+", "-", "/", "3", "*", "2", "1", "1", "+", "+", "-"];
    println!("{}", eval_rpn(&tokens)); // 5
}
