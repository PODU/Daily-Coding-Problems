// Day 88: Integer division using subtraction of shifted divisor (doubling).
// Time O(log^2 q), Space O(1).
fn divide(mut dividend: i64, divisor: i64) -> i64 {
    let mut quotient = 0;
    while dividend >= divisor {
        let mut temp = divisor;
        let mut multiple = 1;
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
    println!("{}", divide(10, 3)); // 3
    println!("{}", divide(27, 4)); // 6
}
