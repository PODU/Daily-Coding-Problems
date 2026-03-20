// Integer division without * / %. Subtract doubled divisor. Time O(log^2 q).
fn divide(mut dividend: i64, divisor: i64) -> i64 {
    let mut quotient = 0i64;
    while dividend >= divisor {
        let mut temp = divisor;
        let mut multiple = 1i64;
        while (temp << 1) <= dividend {
            temp <<= 1;
            multiple <<= 1;
        }
        dividend -= temp;
        quotient += multiple;
    }
    quotient
}

fn main() {
    println!("{}", divide(43, 5));
    println!("{}", divide(100, 10));
}
