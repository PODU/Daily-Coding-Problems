// Day 1294: Run-length encoding and decoding for alphabetic strings.
// Single linear scan for each direction. O(n) time, O(n) space.
fn encode(s: &str) -> String {
    let b = s.as_bytes();
    let n = b.len();
    let mut out = String::new();
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j < n && b[j] == b[i] {
            j += 1;
        }
        out.push_str(&(j - i).to_string());
        out.push(b[i] as char);
        i = j;
    }
    out
}

fn decode(s: &str) -> String {
    let mut out = String::new();
    let mut count = 0usize;
    for c in s.chars() {
        if c.is_ascii_digit() {
            count = count * 10 + c.to_digit(10).unwrap() as usize;
        } else {
            for _ in 0..count {
                out.push(c);
            }
            count = 0;
        }
    }
    out
}

fn main() {
    let e = encode("AAAABBBCCDAA");
    println!("{}", e);            // 4A3B2C1D2A
    println!("{}", decode(&e));   // AAAABBBCCDAA
}
