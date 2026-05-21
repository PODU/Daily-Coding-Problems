// Fisher-Yates shuffle: uniform random permutation using only swaps.
// rand(k) gives a uniform int in [1,k]; each of N! orderings is equally likely.
// Time O(N), Space O(1).
function mulberry32(a) {
  return function () {
    a |= 0;
    a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}
const rng = mulberry32(12345);
// uniform random number between 1 and k inclusive
function randk(k) {
  return Math.floor(rng() * k) + 1;
}

function shuffle(deck) {
  for (let i = deck.length - 1; i > 0; i--) {
    const j = randk(i + 1) - 1; // index in [0, i]
    [deck[i], deck[j]] = [deck[j], deck[i]];
  }
  return deck;
}

const deck = Array.from({ length: 52 }, (_, i) => i + 1);
console.log(shuffle(deck).join(" "));
