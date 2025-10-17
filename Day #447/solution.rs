// Day 447: Integer pow via exponentiation by squaring. O(log y) time, O(1) space.

fn ipow(x: i64, y: i64) -> f64 {
    if y < 0 {
        return 1.0 / ipow(x, -y);
    }
    let mut result: i64 = 1;
    let mut base = x;
    let mut e = y;
    while e > 0 {
        if e & 1 == 1 {
            result *= base;
        }
        base *= base;
        e >>= 1;
    }
    result as f64
}

fn main() {
    println!("{}", ipow(2, 10) as i64); // 1024
    println!("{}", ipow(3, 5) as i64);  // 243
    println!("{}", ipow(2, -2));         // 0.25
}
