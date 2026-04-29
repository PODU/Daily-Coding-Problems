// Day 1441: Validate UTF-8 encoding from an array of byte values.
// Approach: scan bytes, read leading-one count of lead byte, verify
// continuation bytes start with 10. Time O(n), Space O(1).
fn valid_utf8(data: &[i32]) -> bool {
    let n = data.len();
    let mut i = 0;
    while i < n {
        let b = data[i] & 0xFF;
        let cnt = if b >> 7 == 0b0 {
            1
        } else if b >> 5 == 0b110 {
            2
        } else if b >> 4 == 0b1110 {
            3
        } else if b >> 3 == 0b11110 {
            4
        } else {
            return false;
        };
        if i + cnt > n {
            return false;
        }
        for j in 1..cnt {
            if (data[i + j] & 0xFF) >> 6 != 0b10 {
                return false;
            }
        }
        i += cnt;
    }
    true
}

fn main() {
    let euro = [226, 130, 172]; // 11100010 10000010 10101100
    println!("{}", if valid_utf8(&euro) { "True" } else { "False" });
}
