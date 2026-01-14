// Integer division without / * %: subtract largest shifted multiple of divisor.
// Bit-shifting. Time O((log n)^2), Space O(1).
fn divide(mut dividend: i64, divisor: i64) -> i64 {
    let mut quotient = 0i64;
    while dividend >= divisor {
        let mut temp = divisor;
        let mut multiple = 1i64;
        while dividend >= (temp << 1) {
            temp <<= 1;
            multiple <<= 1;
        }
        dividend -= temp;
        quotient += multiple;
    }
    quotient
}

fn main() {
    println!("{}", divide(43, 8));
    println!("{}", divide(100, 9));
}
