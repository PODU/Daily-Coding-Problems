// Prefix-sum hashmap: store sum->index; if sum-K seen, slice between. O(n) time, O(n) space.
use std::collections::HashMap;
fn subarray_sum(a: &[i64], k: i64) -> &[i64] {
    let mut seen: HashMap<i64, i64> = HashMap::new(); seen.insert(0, -1); let mut s = 0i64;
    for i in 0..a.len() {
        s += a[i];
        if let Some(&j) = seen.get(&(s - k)) { return &a[(j + 1) as usize..=i]; }
        seen.entry(s).or_insert(i as i64);
    }
    &[]
}
fn main() {
    let r = subarray_sum(&[1, 2, 3, 4, 5], 9);
    let s: Vec<String> = r.iter().map(|v| v.to_string()).collect();
    println!("[{}]", s.join(", "));
}
