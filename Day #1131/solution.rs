// Blackjack with known deck. DFS over deck index: player hits/stands to maximize
// net (+1/0/-1); dealer then plays fixed rule (hit while <=16). O(deck) states.

const DECK: [i32; 8] = [10, 7, 5, 9, 6, 4, 10, 2];

fn dealer_play(mut p: usize, player_total: i32) -> i32 {
    let mut dealer_total = DECK[1] + DECK[3];
    while dealer_total <= 16 && p < DECK.len() {
        dealer_total += DECK[p];
        p += 1;
    }
    if dealer_total > 21 { return 1; }
    if player_total > dealer_total { return 1; }
    if player_total < dealer_total { return -1; }
    0
}

fn player_play(p: usize, player_total: i32) -> i32 {
    let mut best = dealer_play(p, player_total); // stand
    if p < DECK.len() {
        let nt = player_total + DECK[p];
        let hit = if nt > 21 { -1 } else { player_play(p + 1, nt) };
        best = best.max(hit);
    }
    best
}

fn main() {
    let player_total = DECK[0] + DECK[2];
    println!("{}", player_play(4, player_total));
}
