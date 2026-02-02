// Day 1005: Blackjack solver with perfect knowledge of the deck order.
// Player gets deck[0],deck[2]; dealer gets deck[1],deck[3]; rest is the draw pile.
// Try every number of player hits k, keep best outcome; dealer hits while <= 16.
// O(N^2) over the deck.
fn best_score(deck: &[i32]) -> i32 {
    let n = deck.len();
    let player_base = deck[0] + deck[2];
    let dealer_base = deck[1] + deck[3];
    let mut best = -1;
    let mut k = 0usize;
    loop {
        let mut player = player_base;
        for i in 0..k {
            player += deck[4 + i];
        }
        if player > 21 {
            break;
        }
        let mut idx = 4 + k;
        let mut dealer = dealer_base;
        while dealer <= 16 && idx < n {
            dealer += deck[idx];
            idx += 1;
        }
        let outcome = if dealer > 21 || player > dealer {
            1
        } else if player < dealer {
            -1
        } else {
            0
        };
        best = best.max(outcome);
        if 4 + k >= n {
            break;
        }
        k += 1;
    }
    best
}

fn main() {
    let deck = [10, 10, 6, 9, 5, 7, 3, 8];
    println!("Best player score: {}", best_score(&deck)); // 1
}
