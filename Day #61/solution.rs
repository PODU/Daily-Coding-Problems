// Fast (binary) exponentiation: square-and-multiply. Time O(log y), Space O(1).
fn fast_pow(mut x: i64, mut y: i64) -> i64 {
    if y < 0 {
        y = -y;
    }
    let mut result: i64 = 1;
    while y > 0 {
        if y & 1 == 1 {
            result *= x;
        }
        x *= x;
        y >>= 1;
    }
    result
}

fn main() {
    println!("{}", fast_pow(2, 10)); // 1024
}
