// Minimum coins for {1,5,10,25} via greedy (optimal for this canonical system).
// Time: O(1), Space: O(1).
fn min_coins(n: i32) -> i32 {
    n / 25 + (n % 25) / 10 + (n % 25 % 10) / 5 + (n % 25 % 10 % 5)
}

fn main() {
    println!("{}", min_coins(16));
}
