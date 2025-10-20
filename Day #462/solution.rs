// Day 462: Smallest sparse number (no adjacent set bits) >= N.
// Scan bits low->high, lift each "11" pair upward. Time O(log N), Space O(log N).
fn next_sparse(n: i64) -> i64 {
    if n <= 0 {
        return n;
    }
    let mut bits: Vec<i32> = Vec::new();
    let mut t = n;
    while t > 0 {
        bits.push((t & 1) as i32);
        t >>= 1;
    }
    bits.push(0);
    bits.push(0);
    let size = bits.len();
    let mut last_final = 0i32;
    for i in 1..size - 1 {
        if bits[i] == 1 && bits[i - 1] == 1 && bits[i + 1] == 0 {
            bits[i + 1] = 1;
            let mut j = i as i32;
            while j >= last_final {
                bits[j as usize] = 0;
                j -= 1;
            }
            last_final = i as i32 + 1;
        }
    }
    let mut ans: i64 = 0;
    for i in 0..size {
        if bits[i] == 1 {
            ans |= 1i64 << i;
        }
    }
    ans
}

fn main() {
    println!("{}", next_sparse(22)); // 32
    println!("{}", next_sparse(21)); // 21
}
