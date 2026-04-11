// Min coins for US denominations {1,5,10,25} via greedy (canonical system).
// Time O(#denominations), Space O(1).

fn min_coins(mut n: i32) -> i32 {
    let coins = [25, 10, 5, 1];
    let mut count = 0;
    for c in coins {
        count += n / c;
        n %= c;
    }
    count
}

fn main() {
    println!("{}", min_coins(16));
}
