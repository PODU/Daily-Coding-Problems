// Simplified Elo: expected = 1/(1+10^((Rb-Ra)/400)); delta = round(K*(s-expected)); zero-sum transfer.
// Time O(1), Space O(1).
fn main() {
    let (ra, rb, k) = (1200i32, 2000i32, 32i32);
    let expected_a = 1.0 / (1.0 + 10f64.powf((rb - ra) as f64 / 400.0));
    let delta = (k as f64 * (1.0 - expected_a)).round() as i32; // A wins, s=1
    let new_a = ra + delta;
    let new_b = rb - delta;
    println!("Winner ({}) -> {}, Loser ({}) -> {}", ra, new_a, rb, new_b);
}
