// Day 283: First N regular (5-smooth) numbers via 3-pointer merge of 2x,3x,5x.
// Time O(N), Space O(N).
fn regular(n: usize) -> Vec<i64> {
    let mut h = vec![1i64; n];
    let (mut i2, mut i3, mut i5) = (0usize, 0usize, 0usize);
    for i in 1..n {
        let n2 = h[i2] * 2;
        let n3 = h[i3] * 3;
        let n5 = h[i5] * 5;
        let nxt = n2.min(n3).min(n5);
        h[i] = nxt;
        if nxt == n2 {
            i2 += 1;
        }
        if nxt == n3 {
            i3 += 1;
        }
        if nxt == n5 {
            i5 += 1;
        }
    }
    h
}

fn main() {
    println!("{:?}", regular(10)); // [1, 2, 3, 4, 5, 6, 8, 9, 10, 12]
}
