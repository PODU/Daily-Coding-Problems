// Day 1330: Column number -> spreadsheet label (bijective base-26).
// Repeatedly take (n-1)%26 for the next letter, then n=(n-1)/26. O(log n) time.

fn column_title(mut n: u64) -> String {
    let mut s = Vec::new();
    while n > 0 {
        n -= 1;
        s.push((b'A' + (n % 26) as u8) as char);
        n /= 26;
    }
    s.iter().rev().collect()
}

fn main() {
    println!("\"{}\"", column_title(1));  // "A"
    println!("\"{}\"", column_title(27)); // "AA"
}
