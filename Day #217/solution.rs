// Day 217: Smallest sparse number (no two adjacent set bits) >= N.
// Approach: scan bits low->high; on adjacent 11 with a 0 above, carry up and clear lower bits.
// Time O(bits) ~ O(log N), much faster than O(N log N). Space O(log N).
fn next_sparse(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut bits: Vec<i64> = Vec::new();
    let mut x = n;
    while x != 0 {
        bits.push(x & 1);
        x >>= 1;
    }
    bits.push(0);
    bits.push(0);
    let sz = bits.len();
    let mut last_final = 0;
    for i in 1..sz - 1 {
        if bits[i] == 1 && bits[i - 1] == 1 && bits[i + 1] == 0 {
            bits[i + 1] = 1;
            let mut j = i as i64;
            while j >= last_final {
                bits[j as usize] = 0;
                j -= 1;
            }
            last_final = i as i64 + 1;
        }
    }
    let mut res: i64 = 0;
    for i in (0..sz).rev() {
        res = (res << 1) | bits[i];
    }
    res
}

fn main() {
    println!("{}", next_sparse(22)); // -> 32
    println!("{}", next_sparse(21)); // -> 21
}
