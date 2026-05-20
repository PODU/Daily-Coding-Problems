// Simulate a Markov chain for num_steps and count visits per state.
// Time O(num_steps * outdegree), Space O(states + transitions).
// Uses a small seeded PRNG (mulberry32) for reproducibility.
function mulberry32(a) {
  return function () {
    a |= 0;
    a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function runChain(start, numSteps, transitions, seed = 42) {
  const trans = {};
  for (const [src, dst, p] of transitions) {
    (trans[src] = trans[src] || []).push([dst, p]);
  }
  const rng = mulberry32(seed);
  const counts = {};
  let cur = start;
  for (let s = 0; s < numSteps; s++) {
    counts[cur] = (counts[cur] || 0) + 1;
    let r = rng(), acc = 0;
    for (const [dst, p] of trans[cur]) {
      acc += p;
      if (r <= acc) { cur = dst; break; }
    }
  }
  return counts;
}

const transitions = [
  ["a", "a", 0.9], ["a", "b", 0.075], ["a", "c", 0.025],
  ["b", "a", 0.15], ["b", "b", 0.8], ["b", "c", 0.05],
  ["c", "a", 0.25], ["c", "b", 0.25], ["c", "c", 0.5],
];
const counts = runChain("a", 5000, transitions);
const ordered = {};
for (const k of Object.keys(counts).sort()) ordered[k] = counts[k];
console.log(ordered);
