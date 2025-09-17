// Misere Nim (3 heaps): first player wins iff
// (some heap>1 && xor!=0) || (all heaps<=1 && xor==0). Time: O(1), Space: O(1).
fn first_player_wins(heaps: [i32; 3]) -> bool {
    let [a, b, c] = heaps;
    let x = a ^ b ^ c;
    let any_big = a > 1 || b > 1 || c > 1;
    if any_big {
        x != 0
    } else {
        x == 0
    }
}

fn main() {
    println!("{}", first_player_wins([3, 4, 5]));
}
