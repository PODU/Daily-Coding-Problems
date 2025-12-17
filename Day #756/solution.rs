// Day 756: Recover digits from an anagram of their English spellings.
// Use unique marker letters (z,w,u,x,g) then deduce the rest. Time: O(n), Space: O(1).
fn recover_digits(s: &str) -> String {
    let mut cnt = [0i32; 26];
    for b in s.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }
    let g = |ch: u8| cnt[(ch - b'a') as usize];
    let mut d = [0i32; 10];
    d[0] = g(b'z');
    d[2] = g(b'w');
    d[4] = g(b'u');
    d[6] = g(b'x');
    d[8] = g(b'g');
    d[1] = g(b'o') - d[0] - d[2] - d[4];
    d[3] = g(b'h') - d[8];
    d[5] = g(b'f') - d[4];
    d[7] = g(b's') - d[6];
    d[9] = g(b'i') - d[5] - d[6] - d[8];

    let mut out = String::new();
    for i in 0..10 {
        for _ in 0..d[i] {
            out.push((b'0' + i as u8) as char);
        }
    }
    out
}

fn main() {
    println!("{}", recover_digits("niesevehrtfeev")); // 357
}
