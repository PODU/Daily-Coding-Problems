// Day 716: Validate UTF-8. Read leading byte to get char length (1-4) from its
// high bits, then verify each continuation byte starts with 10. Time O(n).
fn valid_utf8(data: &[i32]) -> bool {
    let n = data.len();
    let mut i = 0;
    while i < n {
        let b = data[i] & 0xFF;
        let len = if b >> 7 == 0b0 {
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
        if i + len > n {
            return false;
        }
        for j in 1..len {
            if (data[i + j] & 0xFF) >> 6 != 0b10 {
                return false;
            }
        }
        i += len;
    }
    true
}

fn b2s(b: bool) -> &'static str {
    if b { "True" } else { "False" }
}

fn main() {
    println!("{}", b2s(valid_utf8(&[226, 130, 172])));
    println!("{}", b2s(valid_utf8(&[235, 140, 4])));
}
