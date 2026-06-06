// Day 1621: First N regular (5-smooth/Hamming) numbers.
// DP merge with 3 pointers for factors 2,3,5. Time O(N), Space O(N).
fn regular_numbers(n: usize) -> Vec<u64> {
    let mut h = vec![1u64; n];
    let (mut i2, mut i3, mut i5) = (0usize, 0usize, 0usize);
    for i in 1..n {
        let (n2, n3, n5) = (h[i2] * 2, h[i3] * 3, h[i5] * 5);
        let m = n2.min(n3).min(n5);
        h[i] = m;
        if m == n2 { i2 += 1; }
        if m == n3 { i3 += 1; }
        if m == n5 { i5 += 1; }
    }
    h
}

fn main() {
    let r = regular_numbers(10);
    let s: Vec<String> = r.iter().map(|v| v.to_string()).collect();
    println!("{}", s.join(" "));
}
