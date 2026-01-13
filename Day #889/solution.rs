// Run-length encoding/decoding. Single pass over the string.
// Time: O(n) encode/decode, Space: O(n) for output.
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
    let b = s.as_bytes();
    let n = b.len();
    let mut out = String::new();
    let mut i = 0;
    while i < n {
        let mut cnt = 0usize;
        while i < n && b[i].is_ascii_digit() {
            cnt = cnt * 10 + (b[i] - b'0') as usize;
            i += 1;
        }
        let c = b[i] as char;
        i += 1;
        for _ in 0..cnt {
            out.push(c);
        }
    }
    out
}

fn main() {
    let input = "AAAABBBCCDAA";
    let enc = encode(input);
    println!("{}", enc);
    println!("{}", decode(&enc));
}
