// Max profit single buy-then-sell: track min price so far and max profit in one pass. O(n) time, O(1) space.

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
    println!("{}", max_profit(&[9, 11, 8, 5, 7, 10]));
}
