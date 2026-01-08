// Evaluate Reverse Polish Notation with a stack. Time O(n), Space O(n).

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
            _ => st.push(t.parse::<i64>().unwrap()),
        }
    }
    *st.last().unwrap()
}

fn main() {
    let tokens = vec![
        "15", "7", "1", "1", "+", "-", "/", "3", "*", "2", "1", "1", "+", "+", "-",
    ];
    println!("{}", eval_rpn(&tokens));
}
