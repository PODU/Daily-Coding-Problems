// Day 1748: Recover digits from an anagram of concatenated number words (zero-nine).
// Approach: unique-letter signatures (z,w,u,x,g first; then derive odd digits). O(n) time, O(1) space.

fn recover(s: &str) -> String {
    let mut c = [0i32; 26];
    for ch in s.bytes() {
        c[(ch - b'a') as usize] += 1;
    }
    let g = |ch: u8| c[(ch - b'a') as usize];
    let mut cnt = [0i32; 10];
    cnt[0] = g(b'z'); // zero
    cnt[2] = g(b'w'); // two
    cnt[4] = g(b'u'); // four
    cnt[6] = g(b'x'); // six
    cnt[8] = g(b'g'); // eight
    cnt[3] = g(b'h') - cnt[8]; // three
    cnt[5] = g(b'f') - cnt[4]; // five
    cnt[7] = g(b's') - cnt[6]; // seven
    cnt[1] = g(b'o') - cnt[0] - cnt[2] - cnt[4]; // one
    cnt[9] = g(b'i') - cnt[5] - cnt[6] - cnt[8]; // nine

    let mut res = String::new();
    for d in 0..=9 {
        for _ in 0..cnt[d] {
            res.push((b'0' + d as u8) as char);
        }
    }
    res
}

fn main() {
    println!("{}", recover("niesevehrtfeev")); // 357
}
