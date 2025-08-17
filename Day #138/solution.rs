// Greedy on canonical US denominations {25,10,5,1}: take largest coin each step.
// Time O(D) where D = #denominations; Space O(1).
fn min_coins(mut n: i32) -> i32 {
    let mut count = 0;
    for c in [25, 10, 5, 1] {
        count += n / c;
        n %= c;
    }
    count
}

fn main() {
    println!("{}", min_coins(16)); // 3
}
