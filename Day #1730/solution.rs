// Day 1730: Fast integer exponentiation (exponentiation by squaring).
// Square the base and halve the exponent each step. Time: O(log y). Space: O(1).

fn fast_pow(x: i64, y: i64) -> i64 {
    if y < 0 {
        // x^(-y) = 1 / x^y; integer result only for x == 1 or -1.
        let inv = fast_pow(x, -y);
        return if inv == 0 { 0 } else { 1 / inv };
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
    result
}

fn main() {
    println!("{}", fast_pow(2, 10)); // 1024
}
