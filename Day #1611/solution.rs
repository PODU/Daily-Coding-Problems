// Orderly Queue: move one of the first k letters to the end repeatedly; find lexicographically smallest.
// k==1 -> smallest rotation; k>=2 -> sorted ascending. Time O(n^2) (k==1) / O(n log n), Space O(n).

fn smallest(s: &str, k: usize) -> String {
    if k >= 2 {
        let mut bytes: Vec<u8> = s.bytes().collect();
        bytes.sort();
        return String::from_utf8(bytes).unwrap();
    }
    // k == 1: smallest rotation
    let n = s.len();
    let mut best = s.to_string();
    for i in 1..n {
        let rot = format!("{}{}", &s[i..], &s[..i]);
        if rot < best {
            best = rot;
        }
    }
    best
}

fn main() {
    println!("{}", smallest("daily", 1));
}
