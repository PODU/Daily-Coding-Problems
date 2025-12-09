// Day 719: Convert 1-based column number to spreadsheet id (bijective base-26).
// Repeatedly take (n-1)%26 then n=(n-1)/26. Time O(log n).
fn col_id(mut n: u64) -> String {
    let mut s = Vec::new();
    while n > 0 {
        n -= 1;
        s.push((b'A' + (n % 26) as u8) as char);
        n /= 26;
    }
    s.iter().rev().collect()
}

fn main() {
    println!("\"{}\"", col_id(1));
    println!("\"{}\"", col_id(27));
}
