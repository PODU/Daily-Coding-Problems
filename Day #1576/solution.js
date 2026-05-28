// Day 1576: Blackjack solver with perfect deck knowledge.
// Player enumerates how many cards to hit (stops if bust); dealer plays forced rules.
// Time: O(N^2) total; Space: O(1).
"use strict";

function dealerPlay(deck, idx, dealerTotal) {
  while (dealerTotal <= 16 && idx < deck.length) dealerTotal += deck[idx++];
  return dealerTotal;
}

function compareScore(player, dealer) {
  if (player > 21) return -1;
  if (dealer > 21) return 1;
  if (player > dealer) return 1;
  if (player < dealer) return -1;
  return 0;
}

function bestScore(deck) {
  let playerTotal = deck[0] + deck[1];
  let best = -Infinity;
  let k = 0;
  while (true) {
    if (playerTotal <= 21) {
      const dealer = dealerPlay(deck, 4 + k, deck[2] + deck[3]);
      best = Math.max(best, compareScore(playerTotal, dealer));
    } else break;
    if (4 + k >= deck.length) break;
    playerTotal += deck[4 + k];
    k++;
  }
  return best;
}

const deck = [10, 6, 9, 7, 5, 10, 2];
console.log("Optimal player score:", bestScore(deck));
