// Day 757: Optimal blackjack with a fully known deck.
// Player draws the next cards off the top; try every stand point, dealer then
// follows the fixed rule. Pick the player's best net score. Time: O(n^2), Space: O(1).
"use strict";

// deck values: 2..10, face=10, ace=1. [0,1]=player, [2,3]=dealer, [4..]=draw pile.
function bestScore(deck) {
    const n = deck.length;
    const player = deck[0] + deck[1];
    const dealerStart = deck[2] + deck[3];
    let best = -Infinity;
    let ptot = player, idx = 4;
    while (true) {
        if (ptot > 21) break;
        let dtot = dealerStart, di = idx;
        while (dtot <= 16 && di < n) dtot += deck[di++];
        let result;
        if (dtot > 21) result = 1;
        else if (ptot > dtot) result = 1;
        else if (ptot < dtot) result = -1;
        else result = 0;
        best = Math.max(best, result);
        if (idx >= n) break;
        ptot += deck[idx++];
    }
    return best;
}

console.log(bestScore([10, 9, 2, 3, 10]));  // 1
