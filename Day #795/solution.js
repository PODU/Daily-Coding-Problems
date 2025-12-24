// Von Neumann debiasing: toss biased coin twice; (0,1)->0, (1,0)->1, else retry.
// Output is provably fair regardless of bias. O(1) expected tosses per fair bit.
let state = 123456789 >>> 0;
function nextRand() { // mulberry32 PRNG in [0,1)
  state = (state + 0x6d2b79f5) >>> 0;
  let t = state;
  t = Math.imul(t ^ (t >>> 15), t | 1);
  t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
  return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
}

function tossBiased() { return nextRand() < 0.3 ? 1 : 0; } // P(1) = 0.3

function fairCoin() {
  while (true) {
    const a = tossBiased();
    const b = tossBiased();
    if (a === 0 && b === 1) return 0;
    if (a === 1 && b === 0) return 1;
  }
}

function main() {
  const N = 100000;
  let ones = 0;
  for (let i = 0; i < N; i++) ones += fairCoin();
  console.log((ones / N).toFixed(1));
}

main();
