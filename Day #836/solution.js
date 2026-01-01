// Day 836: Fisher-Yates shuffle using only a rand(k) RNG (uniform 1..k) and swaps.
// For i=n-1..1: pick j uniform in 0..i via rand(i+1)-1, swap a[i],a[j]. O(N) time, O(1) extra.
// Unbiased: each step picks uniformly among i+1 positions, so all n! permutations are equally likely.

class RNG {
  constructor(seed) { this.state = BigInt(seed) & ((1n << 64n) - 1n); }
  next() {
    this.state = (this.state * 6364136223846793005n + 1442695040888963407n) & ((1n << 64n) - 1n);
    return this.state >> 16n;
  }
  // Uniform in [1, k] with rejection to avoid modulo bias.
  rand(k) {
    const kk = BigInt(k);
    const mask = (1n << 48n) - 1n;
    const limit = (1n << 48n) - ((1n << 48n) % kk);
    while (true) {
      const r = this.next() & mask;
      if (r < limit) return Number(r % kk) + 1;
    }
  }
}

function shuffle(a, rng) {
  for (let i = a.length - 1; i > 0; i--) {
    const j = rng.rand(i + 1) - 1; // uniform 0..i
    [a[i], a[j]] = [a[j], a[i]];
  }
  return a;
}

const deck = Array.from({ length: 52 }, (_, i) => i + 1);
shuffle(deck, new RNG(12345));
console.log(deck.join(" "));
