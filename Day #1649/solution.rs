// Bijective base-26: while n>0, n--, push 'A'+(n%26), n/=26, then reverse. O(log n) time, O(log n) space.
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
    println!("{}", column_title(1));
    println!("{}", column_title(27));
}
