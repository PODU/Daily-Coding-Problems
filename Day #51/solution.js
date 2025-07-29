// Day 51: Uniform shuffle via Fisher-Yates, using only rand(1..k) and swaps.
// Each of n! permutations equally likely. Time: O(n), Space: O(1).

// Perfectly random integer in [1, k].
function randK(k) { return Math.floor(Math.random() * k) + 1; }

function shuffle(deck) {
  for (let i = deck.length - 1; i > 0; i--) {
    const j = randK(i + 1) - 1; // uniform index in [0, i]
    [deck[i], deck[j]] = [deck[j], deck[i]];
  }
}

const deck = Array.from({ length: 52 }, (_, i) => i);
shuffle(deck);
console.log(deck.join(" "));
const seen = [...deck].sort((a, b) => a - b);
console.log("valid permutation:", seen.every((v, i) => v === i));
