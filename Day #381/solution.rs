// Hex string -> bytes -> standard Base64 (with '=' padding).
// Time: O(n), Space: O(n).  Note: canonical Base64 of "deadbeef" is "3q2+7w==".

const B64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn hex_to_base64(h: &str) -> String {
    let bytes: Vec<u32> = (0..h.len())
        .step_by(2)
        .map(|i| u32::from_str_radix(&h[i..i + 2], 16).unwrap())
        .collect();
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let rem = bytes.len() - i;
        let mut n = bytes[i] << 16;
        if rem > 1 { n |= bytes[i + 1] << 8; }
        if rem > 2 { n |= bytes[i + 2]; }
        out.push(B64[((n >> 18) & 63) as usize] as char);
        out.push(B64[((n >> 12) & 63) as usize] as char);
        out.push(if rem > 1 { B64[((n >> 6) & 63) as usize] as char } else { '=' });
        out.push(if rem > 2 { B64[(n & 63) as usize] as char } else { '=' });
        i += 3;
    }
    out
}

fn main() {
    println!("{}", hex_to_base64("deadbeef"));
}
