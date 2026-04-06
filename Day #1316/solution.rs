// Smallest sparse number (no adjacent 1 bits) >= N. Scan bits low->high; at the
// top of each adjacent-ones run, carry up and clear below. Time O(log N).

fn next_sparse(x: u64) -> u64 {
    if x == 0 {
        return 0;
    }
    let mut b: Vec<u8> = Vec::new();
    let mut t = x;
    while t != 0 {
        b.push((t & 1) as u8);
        t >>= 1;
    }
    b.push(0);
    b.push(0); // padding for carries
    let n = b.len();
    for i in 1..n - 1 {
        if b[i] == 1 && b[i - 1] == 1 && b[i + 1] == 0 {
            b[i + 1] = 1;
            for j in 0..=i {
                b[j] = 0;
            }
        }
    }
    let mut ans = 0u64;
    for (i, &bit) in b.iter().enumerate() {
        if bit == 1 {
            ans |= 1u64 << i;
        }
    }
    ans
}

fn main() {
    println!("{}", next_sparse(22)); // 32
}
