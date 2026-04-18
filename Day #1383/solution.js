// Weighted sampling: build cumulative prefix array, draw u in [0,1), binary search to pick. O(n) prep, O(log n) per sample.
// Seeded LCG RNG for deterministic output.
function makeRng(seed) {
  let s = seed >>> 0;
  return function () {
    // Numerical Recipes LCG
    s = (Math.imul(1664525, s) + 1013904223) >>> 0;
    return s / 4294967296;
  };
}

function main() {
  const numbers = [1, 2, 3, 4];
  const probs = [0.1, 0.5, 0.2, 0.2];
  const n = numbers.length;

  const cum = [];
  let acc = 0;
  for (const p of probs) { acc += p; cum.push(acc); }

  const rng = makeRng(42);
  const N = 100000;
  const counts = new Array(n).fill(0);
  for (let s = 0; s < N; s++) {
    const u = rng();
    let lo = 0, hi = n - 1;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (cum[mid] < u) lo = mid + 1;
      else hi = mid;
    }
    counts[lo]++;
  }

  for (let i = 0; i < n; i++)
    console.log(`${numbers[i]}: ${(counts[i] / N).toFixed(2)}`);
}

main();
