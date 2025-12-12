// First N regular (5-smooth / Hamming) numbers via 3-pointer dynamic programming.
// Each number is min of next multiples by 2, 3, 5.
// Time: O(N), Space: O(N).

fn regular_numbers(n: usize) -> Vec<u64> {
    let mut h = vec![1u64; n];
    let (mut i2, mut i3, mut i5) = (0usize, 0usize, 0usize);
    for i in 1..n {
        let n2 = h[i2] * 2;
        let n3 = h[i3] * 3;
        let n5 = h[i5] * 5;
        let nx = n2.min(n3).min(n5);
        h[i] = nx;
        if nx == n2 {
            i2 += 1;
        }
        if nx == n3 {
            i3 += 1;
        }
        if nx == n5 {
            i5 += 1;
        }
    }
    h
}

fn main() {
    let r = regular_numbers(10);
    let parts: Vec<String> = r.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" ")); // 1 2 3 4 5 6 8 9 10 12
}
