// Egyptian fraction via Fibonacci/Sylvester greedy: repeatedly subtract the
// largest unit fraction 1/ceil(b/a). Time O(#terms), Space O(1).

fn egyptian(mut a: i64, mut b: i64) -> String {
    let mut terms = Vec::new();
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
    println!("{}", egyptian(4, 13)); // 1 / 4 + 1 / 18 + 1 / 468
}
