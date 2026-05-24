// Max profit single buy-then-sell: track running min price and best (price - min). Time O(n), Space O(1).

fn max_profit(prices: &[i32]) -> i32 {
    if prices.is_empty() {
        return 0;
    }
    let mut min_price = prices[0];
    let mut best = 0;
    for &p in prices {
        if p < min_price {
            min_price = p;
        }
        if p - min_price > best {
            best = p - min_price;
        }
    }
    best
}

fn main() {
    let prices = [9, 11, 8, 5, 7, 10];
    println!("{}", max_profit(&prices));
}
