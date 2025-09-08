// Blackjack with known deck: deal alternately (player,dealer,player,dealer), then the player
// may take k>=0 hits. Try every k (until bust), simulate the forced dealer, pick the best net
// score (+1 win / 0 push / -1 loss). Time: O(deck), Space: O(1).
function bestScore(deck) {
  const player2 = deck[0] + deck[2];
  const dealer2 = deck[1] + deck[3];
  let best = -Infinity;
  let psum = player2, idx = 4;
  while (true) {
    let outcome;
    if (psum > 21) {
      outcome = -1;
    } else {
      let dsum = dealer2, di = idx;
      while (dsum <= 16 && di < deck.length) dsum += deck[di++];
      if (dsum > 21 || psum > dsum) outcome = 1;
      else if (psum < dsum) outcome = -1;
      else outcome = 0;
    }
    best = Math.max(best, outcome);
    if (psum > 21 || idx >= deck.length) break;
    psum += deck[idx++];
  }
  return best;
}

const deck = [5, 10, 6, 9, 10, 2, 3, 7, 8, 4];
console.log("Best score: " + bestScore(deck)); // 1
