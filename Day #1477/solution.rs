// Day 1477: First N regular numbers (only prime factors 2, 3, 5).
// DP with three pointers merging *2, *3, *5 sequences. Time O(N), Space O(N).

fn regular_numbers(n: usize) -> Vec<i64> {
    if n == 0 {
        return vec![];
    }
    let mut h = vec![1i64; n];
    let (mut i2, mut i3, mut i5) = (0usize, 0usize, 0usize);
    for k in 1..n {
        let nxt = (h[i2] * 2).min(h[i3] * 3).min(h[i5] * 5);
        h[k] = nxt;
        if nxt == h[i2] * 2 {
            i2 += 1;
        }
        if nxt == h[i3] * 3 {
            i3 += 1;
        }
        if nxt == h[i5] * 5 {
            i5 += 1;
        }
    }
    h
}

fn main() {
    println!("{:?}", regular_numbers(10)); // [1, 2, 3, 4, 5, 6, 8, 9, 10, 12]
}
