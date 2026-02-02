// Day 1005: Blackjack solver with perfect knowledge of the deck order.
// Player gets deck[0],deck[2]; dealer gets deck[1],deck[3]; rest is the draw pile.
// Try every number of player hits k, keep best outcome; dealer hits while <= 16.
// O(N^2) over the deck.

function bestScore(deck) {
  const n = deck.length;
  const playerBase = deck[0] + deck[2];
  const dealerBase = deck[1] + deck[3];
  let best = -1;
  for (let k = 0; ; k++) {
    let player = playerBase;
    for (let i = 0; i < k; i++) player += deck[4 + i];
    if (player > 21) break;
    let idx = 4 + k,
      dealer = dealerBase;
    while (dealer <= 16 && idx < n) dealer += deck[idx++];
    const outcome = dealer > 21 || player > dealer ? 1 : player < dealer ? -1 : 0;
    best = Math.max(best, outcome);
    if (4 + k >= n) break;
  }
  return best;
}

const deck = [10, 10, 6, 9, 5, 7, 3, 8];
console.log("Best player score:", bestScore(deck)); // 1
