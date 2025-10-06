// Closest coin by Manhattan distance. Linear scan.
// Time: O(n), Space: O(1).
fn closest_coin(me: (i64, i64), coins: &[(i64, i64)]) -> (i64, i64) {
    *coins
        .iter()
        .min_by_key(|c| (c.0 - me.0).abs() + (c.1 - me.1).abs())
        .unwrap()
}

fn main() {
    let me = (0, 2);
    let coins = [(0, 4), (1, 0), (2, 0), (3, 2)];
    let b = closest_coin(me, &coins);
    println!("({}, {})", b.0, b.1);
}
