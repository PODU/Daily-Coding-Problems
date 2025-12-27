// Day 805: Spreadsheet column number -> alphabetical label (bijective base 26).
// Repeatedly take (n-1)%26 for the letter, then n=(n-1)/26. Time O(log n), Space O(log n).

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
    println!("\"{}\"", column_label(1));  // "A"
    println!("\"{}\"", column_label(27)); // "AA"
}
