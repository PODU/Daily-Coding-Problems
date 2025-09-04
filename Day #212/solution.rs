// Day 212: Spreadsheet column number -> label (bijective base-26).
// Approach: repeatedly take (n-1)%26 for the digit, then n=(n-1)/26. Time O(log n), Space O(log n).
fn column_label(mut n: u64) -> String {
    let mut s = Vec::new();
    while n > 0 {
        n -= 1;
        s.push((b'A' + (n % 26) as u8) as char);
        n /= 26;
    }
    s.iter().rev().collect()
}

fn main() {
    println!("\"{}\"", column_label(1));
    println!("\"{}\"", column_label(27));
}
