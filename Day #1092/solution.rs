// First N regular (Hamming) numbers via 3-pointer merge of {2,3,5} multiples. Time O(N), Space O(N).
fn regular(n: usize) -> Vec<i64> {
    let mut h = vec![1i64; n];
    let (mut i2, mut i3, mut i5) = (0usize, 0usize, 0usize);
    for i in 1..n {
        let (n2, n3, n5) = (h[i2] * 2, h[i3] * 3, h[i5] * 5);
        let m = n2.min(n3).min(n5);
        h[i] = m;
        if m == n2 {
            i2 += 1;
        }
        if m == n3 {
            i3 += 1;
        }
        if m == n5 {
            i5 += 1;
        }
    }
    h
}

fn main() {
    println!("{:?}", regular(10));
}
