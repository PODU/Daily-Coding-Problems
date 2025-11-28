// Day 669: Misere Nim. First player wins iff either (some heap>1 and xor!=0) or
// (all heaps<=1 and xor==0). Time O(n), Space O(1).
fn first_player_wins(heaps: &[i32]) -> bool {
    let x = heaps.iter().fold(0, |a, &b| a ^ b);
    let any_big = heaps.iter().any(|&h| h > 1);
    if any_big {
        x != 0
    } else {
        x == 0
    }
}

fn main() {
    println!("{}", if first_player_wins(&[3, 4, 5]) { "True" } else { "False" }); // True
}
