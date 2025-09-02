// Day 202: Integer palindrome check without string conversion.
// Reverse the second half of the digits and compare with the first half.
// Time: O(log10 n), Space: O(1).
fn is_palindrome(mut x: i64) -> bool {
    if x < 0 {
        return false;
    }
    if x != 0 && x % 10 == 0 {
        return false; // trailing zero, not 0 itself
    }
    let mut rev = 0i64;
    while x > rev {
        rev = rev * 10 + x % 10;
        x /= 10;
    }
    x == rev || x == rev / 10
}

fn main() {
    println!("{}", is_palindrome(121)); // true
    println!("{}", is_palindrome(888)); // true
    println!("{}", is_palindrome(678)); // false
}
