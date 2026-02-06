// Day 1029: Minimum coins for n cents with {1,5,10,25}. Greedy is optimal for
// this canonical US denomination set. Time O(#denominations), Space O(1).
fn min_coins(mut n: u32) -> u32 {
    let mut count = 0;
    for c in [25, 10, 5, 1] {
        count += n / c;
        n %= c;
    }
    count
}

fn main() {
    println!("{}", min_coins(16));
}
