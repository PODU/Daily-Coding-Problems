// Brute-force all 256 single-byte keys; score by letters/spaces to pick plaintext.
// Time: O(256 * n), Space: O(n).

fn hex_to_bytes(h: &str) -> Vec<u8> {
    (0..h.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&h[i..i + 2], 16).unwrap())
        .collect()
}

fn score(codes: &[u8]) -> i32 {
    if codes.iter().any(|&c| c < 32 || c > 126) {
        return -1;
    }
    codes
        .iter()
        .filter(|&&c| c.is_ascii_alphabetic() || c == b' ')
        .count() as i32
}

fn decrypt(hex: &str) -> String {
    let bytes = hex_to_bytes(hex);
    let mut best = String::new();
    let mut best_score = -1;
    for k in 0u16..256 {
        let codes: Vec<u8> = bytes.iter().map(|&b| b ^ (k as u8)).collect();
        let sc = score(&codes);
        if sc > best_score {
            best_score = sc;
            best = String::from_utf8_lossy(&codes).to_string();
        }
    }
    best
}

fn main() {
    let h = "7a575e5e5d12455d405e561254405d5f1276535b5e4b12715d565b5c551262405d505e575f";
    println!("{}", decrypt(h));
}
