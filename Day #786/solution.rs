// Integer exponentiation by squaring. Time O(log y), Space O(1).
// Demo uses pow(2,10).

fn ipow(x: i64, y: i64) -> i64 {
    let mut result: i64 = 1;
    let mut base = x;
    let mut e = y;
    while e > 0 {
        if e & 1 == 1 {
            result *= base;
        }
        base = base.wrapping_mul(base);
        e >>= 1;
    }
    result
}

fn main() {
    println!("{}", ipow(2, 10));
}
