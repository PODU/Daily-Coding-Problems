// Egyptian fraction via greedy: repeatedly take ceil(b/a) as next unit denominator.
// Time: O(number of terms) iterations; Space: O(terms). a/b proper (a<b).

fn egyptian(mut a: i64, mut b: i64) -> Vec<i64> {
    let mut denoms = Vec::new();
    while a != 0 {
        let d = (b + a - 1) / a; // ceil(b/a)
        denoms.push(d);
        a = a * d - b;
        b = b * d;
    }
    denoms
}

fn main() {
    let denoms = egyptian(4, 13);
    let parts: Vec<String> = denoms.iter().map(|d| format!("1 / {}", d)).collect();
    println!("{}", parts.join(" + "));
}
