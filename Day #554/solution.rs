// Egyptian fraction via greedy (Fibonacci/Sylvester): take ceil(b/a) each step.
// Time: O(number of terms), Space: O(1). i64.
fn egyptian(mut a: i64, mut b: i64) -> String {
    let mut terms: Vec<String> = Vec::new();
    while a != 0 {
        let x = (b + a - 1) / a; // ceil(b/a)
        terms.push(format!("1 / {}", x));
        a = a * x - b;
        b = b * x;
    }
    terms.join(" + ")
}

fn main() {
    println!("{}", egyptian(4, 13));
}
