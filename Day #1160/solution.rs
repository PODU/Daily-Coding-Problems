// Recover digits from scrambled English spellings using unique identifying letters.
// Time: O(L), Space: O(1).
fn recover(s: &str) -> String {
    let mut c = [0i32; 26];
    for ch in s.bytes() {
        c[(ch - b'a') as usize] += 1;
    }
    let g = |ch: u8| c[(ch - b'a') as usize];
    let mut cnt = [0i32; 10];
    cnt[0] = g(b'z');
    cnt[2] = g(b'w');
    cnt[4] = g(b'u');
    cnt[6] = g(b'x');
    cnt[8] = g(b'g');
    cnt[3] = g(b'h') - cnt[8];
    cnt[5] = g(b'f') - cnt[4];
    cnt[7] = g(b's') - cnt[6];
    cnt[1] = g(b'o') - cnt[0] - cnt[2] - cnt[4];
    cnt[9] = g(b'i') - cnt[5] - cnt[6] - cnt[8];
    let mut out = String::new();
    for d in 0..10 {
        for _ in 0..cnt[d] {
            out.push((b'0' + d as u8) as char);
        }
    }
    out
}

fn main() {
    println!("{}", recover("niesevehrtfeev"));
}
