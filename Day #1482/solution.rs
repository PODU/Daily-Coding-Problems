// Day 1482: Integer palindrome without converting to a string.
// Reverse the number arithmetically and compare. Time O(digits), Space O(1).

fn is_palindrome(x: i64) -> bool {
    if x < 0 {
        return false;
    }
    let (mut rev, mut n) = (0i64, x);
    while n > 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }
    rev == x
}

fn main() {
    for v in [121i64, 888, 678] {
        let label = if is_palindrome(v) { "palindrome" } else { "not a palindrome" };
        println!("{} -> {}", v, label);
    }
}
