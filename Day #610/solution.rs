// Day 610: Integer division of positive ints without / , * , or %.
// Approach: repeated doubling subtraction (binary long division). Time O(log^2), Space O(1).

fn divide(mut dividend: i64, divisor: i64) -> i64 {
    let mut q = 0i64;
    while dividend >= divisor {
        let mut temp = divisor;
        let mut mult = 1i64;
        while dividend >= (temp << 1) {
            temp <<= 1;
            mult <<= 1;
        }
        dividend -= temp;
        q += mult;
    }
    q
}

fn main() {
    println!("{}", divide(10, 3)); // 3
    println!("{}", divide(43, 8)); // 5
}
