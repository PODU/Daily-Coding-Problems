// Day 541: Run-length encoding/decoding. Scan runs to build count+char; parse digits to expand.
// Time O(n) encode, O(output) decode. Space O(n).

fn encode(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        let mut j = i;
        while j < chars.len() && chars[j] == chars[i] {
            j += 1;
        }
        out.push_str(&(j - i).to_string());
        out.push(chars[i]);
        i = j;
    }
    out
}

fn decode(s: &str) -> String {
    let mut out = String::new();
    let mut count = 0usize;
    for c in s.chars() {
        if let Some(d) = c.to_digit(10) {
            count = count * 10 + d as usize;
        } else {
            out.push_str(&c.to_string().repeat(count));
            count = 0;
        }
    }
    out
}

fn main() {
    let original = "AAAABBBCCDAA";
    let enc = encode(original);
    println!("{}", enc); // 4A3B2C1D2A
    println!("{}", decode(&enc) == original);
}
