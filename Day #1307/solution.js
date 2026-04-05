// Fisher-Yates shuffle using a uniform rand(1..k). Each of N! permutations
// equally likely. Time O(N), Space O(1) extra.
'use strict';

// Seeded LCG so the demo is reproducible.
let seed = 12345;
function rand() { seed = (seed * 1103515245 + 12345) & 0x7fffffff; return seed; }
function randK(k) { return (rand() % k) + 1; } // uniform in [1, k]

function shuffleDeck(deck) {
  for (let i = deck.length - 1; i > 0; i--) {
    const j = randK(i + 1) - 1;      // uniform in [0, i]
    [deck[i], deck[j]] = [deck[j], deck[i]];
  }
  return deck;
}

const deck = Array.from({ length: 52 }, (_, i) => i + 1);
shuffleDeck(deck);
console.log(deck.join(' '));
