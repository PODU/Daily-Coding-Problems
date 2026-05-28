// Day 1576: Blackjack solver with perfect deck knowledge.
// Player enumerates how many cards to hit (stops if bust); dealer plays forced rules.
// Time: O(N^2) total; Space: O(1).

fn dealer_play(deck: &[i32], mut idx: usize, mut dealer_total: i32) -> i32 {
    while dealer_total <= 16 && idx < deck.len() {
        dealer_total += deck[idx];
        idx += 1;
    }
    dealer_total
}

fn compare_score(player: i32, dealer: i32) -> i32 {
    if player > 21 {
        return -1;
    }
    if dealer > 21 {
        return 1;
    }
    if player > dealer {
        1
    } else if player < dealer {
        -1
    } else {
        0
    }
}

fn best_score(deck: &[i32]) -> i32 {
    let mut player_total = deck[0] + deck[1];
    let mut best = i32::MIN;
    let mut k = 0usize;
    loop {
        if player_total <= 21 {
            let dealer = dealer_play(deck, 4 + k, deck[2] + deck[3]);
            best = best.max(compare_score(player_total, dealer));
        } else {
            break;
        }
        if 4 + k >= deck.len() {
            break;
        }
        player_total += deck[4 + k];
        k += 1;
    }
    best
}

fn main() {
    let deck = [10, 6, 9, 7, 5, 10, 2];
    println!("Optimal player score: {}", best_score(&deck));
}
