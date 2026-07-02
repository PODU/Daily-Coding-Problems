// Day 1753: Egyptian fraction (sum of distinct unit fractions).
// Greedy Fibonacci-Sylvester: repeatedly subtract 1/ceil(b/a). Time O(terms), O(1) space.

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let t = x % y;
        x = y;
        y = t;
    }
    x
}

fn egyptian(mut a: i64, mut b: i64) -> String {
    let mut terms: Vec<String> = Vec::new();
    while a != 0 {
        let c = (b + a - 1) / a; // ceil(b / a)
        terms.push(format!("1 / {}", c));
        a = a * c - b;
        b = b * c;
        let g = gcd(a.abs(), b);
        if g > 1 {
            a /= g;
            b /= g;
        }
    }
    terms.join(" + ")
}

fn main() {
    // README example: 4 / 13 -> 1 / 4 + 1 / 18 + 1 / 468
    println!("{}", egyptian(4, 13));
}
