// Day 1677: Linked-list palindrome.
// Idiomatic Rust uses a Vec for the list; check via two-pointer compare. Time O(n).
fn is_palindrome(values: &[i32]) -> bool {
    let n = values.len();
    for i in 0..n / 2 {
        if values[i] != values[n - 1 - i] {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", is_palindrome(&[1, 4, 3, 4, 1])); // true
    println!("{}", is_palindrome(&[1, 4]));          // false
}
