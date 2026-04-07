// Day 1322: Misere Nim (last stone loses) forced win for first player.
// Theorem: first player wins iff (some heap>1 and XOR!=0) OR (all heaps<=1 and XOR==0). O(n) time, O(1) space.

fn first_player_wins(heaps: &[i32]) -> bool {
    let x = heaps.iter().fold(0, |a, &b| a ^ b);
    let max_heap = *heaps.iter().max().unwrap_or(&0);
    if max_heap <= 1 {
        x == 0
    } else {
        x != 0
    }
}

fn main() {
    println!("{}", first_player_wins(&[3, 4, 5])); // true
}
