// Misere Nim: P-position (loss for mover) iff (all heaps==1 with even count) or (some heap>1 and xor==0). First wins otherwise.
// Time O(n), Space O(1).
fn first_player_wins(heaps: &[i32]) -> bool {
    let x = heaps.iter().fold(0, |a, &b| a ^ b);
    let mx = *heaps.iter().max().unwrap_or(&0);
    let p_position = if mx <= 1 {
        x == 0 // all heaps == 1: P iff even count
    } else {
        x == 0 // some heap > 1: P iff xor == 0
    };
    !p_position
}

fn main() {
    let heaps = [3, 4, 5];
    println!("{}", first_player_wins(&heaps)); // expected true
}
