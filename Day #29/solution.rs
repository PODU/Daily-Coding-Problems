// Run-length encoding/decoding in a single pass.
// Time: O(n), Space: O(n) for output.
fn encode(s: &str) -> String {
    let b = s.as_bytes();
    let n = b.len();
    let mut res = String::new();
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j < n && b[j] == b[i] {
            j += 1;
        }
        res.push_str(&(j - i).to_string());
        res.push(b[i] as char);
        i = j;
    }
    res
}

fn decode(s: &str) -> String {
    let b = s.as_bytes();
    let n = b.len();
    let mut res = String::new();
    let mut i = 0;
    while i < n {
        let mut count = 0usize;
        while i < n && b[i].is_ascii_digit() {
            count = count * 10 + (b[i] - b'0') as usize;
            i += 1;
        }
        let c = b[i] as char;
        i += 1;
        for _ in 0..count {
            res.push(c);
        }
    }
    res
}

fn main() {
    let input = "AAAABBBCCDAA";
    let enc = encode(input);
    println!("{}", enc);
    println!("{}", decode(&enc));
}
