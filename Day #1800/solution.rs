// Palindrome integer check by reversing the number arithmetically (no string). Negatives are not palindromes. O(log10(n)).
fn is_palindrome(x: i64) -> bool {
    if x < 0 {
        return false;
    }
    let original = x;
    let mut n = x;
    let mut reversed = 0i64;
    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }
    reversed == original
}

fn main() {
    println!("{}", is_palindrome(121)); // true
}
