// Bloom filter: fixed 1000-bit array, 3 hashes via double hashing. add/check.
// add O(k), check O(k); Space O(m bits). check may false-positive, never false-negative.
const M: usize = 1000;
const K: usize = 3;

struct Bloom {
    bits: [bool; M],
}
impl Bloom {
    fn new() -> Self { Bloom { bits: [false; M] } }
    fn h1(s: &str) -> u64 { let mut h: u64 = 5381; for c in s.bytes() { h = h.wrapping_mul(33).wrapping_add(c as u64); } h }
    fn h2(s: &str) -> u64 { let mut h: u64 = 0; for c in s.bytes() { h = h.wrapping_mul(131).wrapping_add(c as u64); } h }
    fn add(&mut self, s: &str) {
        let (a, b) = (Self::h1(s), Self::h2(s));
        for i in 0..K as u64 { self.bits[(a.wrapping_add(i.wrapping_mul(b)) % M as u64) as usize] = true; }
    }
    fn check(&self, s: &str) -> bool {
        let (a, b) = (Self::h1(s), Self::h2(s));
        (0..K as u64).all(|i| self.bits[(a.wrapping_add(i.wrapping_mul(b)) % M as u64) as usize])
    }
}
fn main() {
    let mut bf = Bloom::new();
    for w in ["apple", "banana", "cat"] { bf.add(w); }
    for w in ["apple", "banana", "cat", "dog"] { println!("check {}: {}", w, bf.check(w)); }
}
