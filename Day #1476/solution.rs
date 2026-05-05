// Day 1476: Max profit from a single buy-then-sell.
// Track the minimum price so far and the best profit in one pass.
// Time O(N), Space O(1).

fn max_profit(prices: &[i32]) -> i32 {
    let mut min_price = i32::MAX;
    let mut best = 0;
    for &p in prices {
        if p < min_price {
            min_price = p;
        } else if p - min_price > best {
            best = p - min_price;
        }
    }
    best
}

fn main() {
    println!("{}", max_profit(&[9, 11, 8, 5, 7, 10])); // 5
}
