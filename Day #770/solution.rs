// Day 770: Misere Nim forced-win check.
// If every heap == 1: first player wins iff count of heaps is even.
// Else: first player wins iff XOR of heaps != 0. O(N).
fn first_player_wins(heaps: &[i32]) -> bool {
    let mut xor_sum = 0;
    let mut all_one = true;
    for &h in heaps {
        xor_sum ^= h;
        if h > 1 {
            all_one = false;
        }
    }
    if all_one {
        heaps.len() % 2 == 0
    } else {
        xor_sum != 0
    }
}

fn main() {
    println!("{}", first_player_wins(&[3, 4, 5])); // true
}
