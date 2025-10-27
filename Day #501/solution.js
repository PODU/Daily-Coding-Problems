// Fisher-Yates shuffle using a rand(k) helper returning [1,k]; O(N) time, O(1) extra space.
// Each of the N! permutations is equally likely. Fixed seed -> reproducible output.

// Deterministic seeded PRNG (mulberry32) so output is stable.
function makeRng(seed) {
  let a = seed >>> 0;
  return function () {
    a |= 0;
    a = (a + 0x6D2B79F5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

const rng = makeRng(12345);

// Uniform integer in [1, k].
function randk(k) {
  return Math.floor(rng() * k) + 1;
}

function shuffleDeck(arr) {
  const n = arr.length;
  for (let i = n - 1; i >= 1; i--) {
    const j = randk(i + 1) - 1; // index in [0, i]
    [arr[i], arr[j]] = [arr[j], arr[i]];
  }
}

const deck = Array.from({ length: 52 }, (_, i) => i + 1); // cards 1..52
shuffleDeck(deck);
console.log(deck.join(" "));
