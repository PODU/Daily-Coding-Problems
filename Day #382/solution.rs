// Base64 -> bytes -> hex. Bit-accumulator decode (tolerates padding/whitespace).
// Time: O(n), Space: O(n).

const B64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn base64_to_hex(s: &str) -> String {
    let mut val = [-1i32; 256];
    for (i, &c) in B64.iter().enumerate() {
        val[c as usize] = i as i32;
    }
    let (mut bits, mut nbits) = (0u64, 0i32);
    let mut out = String::new();
    for c in s.bytes() {
        let v = val[c as usize];
        if v < 0 {
            continue;
        }
        bits = (bits << 6) | v as u64;
        nbits += 6;
        if nbits >= 8 {
            nbits -= 8;
            let b = (bits >> nbits) & 0xFF;
            out.push_str(&format!("{:02x}", b));
        }
    }
    out
}

fn main() {
    println!("{}", base64_to_hex("3q2+7w="));
}
