// Day 731: Max profit from a single buy-then-sell.
// Approach: Track running minimum price and best profit in one pass.
// Time: O(n), Space: O(1).

fn max_profit(prices: &[i32]) -> i32 {
    let mut min_price = i32::MAX;
    let mut best = 0;
    for &p in prices {
        min_price = min_price.min(p);
        best = best.max(p - min_price);
    }
    best
}

fn main() {
    println!("{}", max_profit(&[9, 11, 8, 5, 7, 10])); // 5
}
