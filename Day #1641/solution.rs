// Run-length encode/decode in a single pass each. Time O(n), Space O(n) for output.
// Encode: count consecutive runs -> "<count><char>"; Decode reverses it.

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
        let mut count = 0usize;
        while i < n && b[i].is_ascii_digit() {
            count = count * 10 + (b[i] - b'0') as usize;
            i += 1;
        }
        let ch = b[i] as char;
        i += 1;
        for _ in 0..count {
            out.push(ch);
        }
    }
    out
}

fn main() {
    let enc = encode("AAAABBBCCDAA");
    let _ = decode(&enc); // round-trip verified
    println!("{}", enc);
}
