// Egyptian fraction via greedy: take ceil(b/a) each step. Time O(terms), Space O(1).
fn egyptian(mut a: i64, mut b: i64) -> String {
    let mut terms: Vec<String> = Vec::new();
    while a != 0 {
        let x = (b + a - 1) / a; // ceil(b/a)
        terms.push(format!("1 / {}", x));
        let na = a * x - b;
        b *= x;
        a = na;
    }
    terms.join(" + ")
}

fn main() {
    println!("{}", egyptian(4, 13));
}
