// Integer palindrome check by arithmetic reversal (no string). O(log n) time, O(1) space. Negatives -> false.
fn is_palindrome(n: i64) -> bool {
    if n < 0 {
        return false;
    }
    let mut rev = 0i64;
    let mut x = n;
    while x > 0 {
        rev = rev * 10 + x % 10;
        x /= 10;
    }
    rev == n
}

fn main() {
    println!("{}", is_palindrome(121));
}
