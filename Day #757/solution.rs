// Day 757: Optimal blackjack with a fully known deck.
// Player draws the next cards off the top; try every stand point, dealer then
// follows the fixed rule. Pick the player's best net score. Time: O(n^2), Space: O(1).

// deck values: 2..10, face=10, ace=1. [0,1]=player, [2,3]=dealer, [4..]=draw pile.
fn best_score(deck: &[i32]) -> i32 {
    let n = deck.len();
    let player = deck[0] + deck[1];
    let dealer_start = deck[2] + deck[3];
    let mut best = i32::MIN;
    let mut ptot = player;
    let mut idx = 4;
    loop {
        if ptot > 21 {
            break;
        }
        let mut dtot = dealer_start;
        let mut di = idx;
        while dtot <= 16 && di < n {
            dtot += deck[di];
            di += 1;
        }
        let result = if dtot > 21 {
            1
        } else if ptot > dtot {
            1
        } else if ptot < dtot {
            -1
        } else {
            0
        };
        best = best.max(result);
        if idx >= n {
            break;
        }
        ptot += deck[idx];
        idx += 1;
    }
    best
}

fn main() {
    let deck = [10, 9, 2, 3, 10];
    println!("{}", best_score(&deck)); // 1
}
