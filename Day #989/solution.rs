// Day 989: Deduce coin denominations from a ways-to-make-change array.
// Walk amounts; whenever target[i] exceeds reconstructed ways, i is a coin and we fold it into the DP.
// O(N^2) time, O(N) space.

fn find_denominations(target: &[i64]) -> Vec<usize> {
    let n = target.len();
    let mut have = vec![0i64; n];
    have[0] = 1; // one way to make 0 with no coins
    let mut coins = Vec::new();
    for i in 1..n {
        if target[i] > have[i] { // unaccounted combinations => i is a denomination
            coins.push(i);
            for j in i..n {
                have[j] += have[j - i];
            }
        }
    }
    coins
}

fn main() {
    let target = [1i64, 0, 1, 1, 2];
    let coins = find_denominations(&target);
    let parts: Vec<String> = coins.iter().map(|c| c.to_string()).collect();
    println!("{}", parts.join(", ")); // expected: 2, 3, 4
}
