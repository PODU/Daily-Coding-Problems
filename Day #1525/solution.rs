// UTF-8 validation: read lead byte, count leading ones (1->1byte, 2..4 multi), verify continuation bytes start with 10.
// Time O(n), Space O(1).
fn valid_utf8(data: &[i32]) -> bool {
    let n = data.len();
    let mut i = 0;
    while i < n {
        let b = data[i] & 0xFF;
        let cnt = if b & 0x80 == 0x00 {
            1
        } else if b & 0xE0 == 0xC0 {
            2
        } else if b & 0xF0 == 0xE0 {
            3
        } else if b & 0xF8 == 0xF0 {
            4
        } else {
            return false;
        };
        if i + cnt > n {
            return false;
        }
        for k in 1..cnt {
            if data[i + k] & 0xC0 != 0x80 {
                return false;
            }
        }
        i += cnt;
    }
    true
}

fn main() {
    println!("{}", valid_utf8(&[226, 130, 172]));                       // true
    println!("{}", valid_utf8(&[0b11110101, 0b10000010, 0b00000010]));  // false
}
