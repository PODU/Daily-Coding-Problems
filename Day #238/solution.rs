// Blackjack with known deck: deal alternately (player,dealer,player,dealer), then the player
// may take k>=0 hits. Try every k (until bust), simulate the forced dealer, pick the best net
// score (+1 win / 0 push / -1 loss). Time: O(deck), Space: O(1).
fn best_score(deck: &[i32]) -> i32 {
    let player2 = deck[0] + deck[2];
    let dealer2 = deck[1] + deck[3];
    let mut best = i32::MIN;
    let (mut psum, mut idx) = (player2, 4usize);
    loop {
        let outcome;
        if psum > 21 {
            outcome = -1;
        } else {
            let (mut dsum, mut di) = (dealer2, idx);
            while dsum <= 16 && di < deck.len() {
                dsum += deck[di];
                di += 1;
            }
            outcome = if dsum > 21 || psum > dsum {
                1
            } else if psum < dsum {
                -1
            } else {
                0
            };
        }
        best = best.max(outcome);
        if psum > 21 || idx >= deck.len() {
            break;
        }
        psum += deck[idx];
        idx += 1;
    }
    best
}

fn main() {
    let deck = [5, 10, 6, 9, 10, 2, 3, 7, 8, 4];
    println!("Best score: {}", best_score(&deck)); // 1
}
