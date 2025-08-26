// Markov chain simulation: sample next state via cumulative probabilities, fixed-seed RNG.
// Time O(num_steps * avg_outdegree), Space O(states). (Exact counts depend on RNG.)
'use strict';

function makeRng(seed) {
  // Mulberry32 deterministic PRNG.
  let a = seed >>> 0;
  return function () {
    a |= 0; a = (a + 0x6D2B79F5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function simulate(start, numSteps, transitions, seed = 42) {
  const trans = new Map();
  for (const [src, dst, prob] of transitions) {
    if (!trans.has(src)) trans.set(src, []);
    trans.get(src).push([dst, prob]);
  }
  const rng = makeRng(seed);
  const counts = {};
  let state = start;
  for (let i = 0; i < numSteps; i++) {
    counts[state] = (counts[state] || 0) + 1;
    const r = rng();
    let cum = 0;
    for (const [dst, prob] of trans.get(state)) {
      cum += prob;
      if (r < cum) { state = dst; break; }
    }
  }
  return counts;
}

const transitions = [
  ['a', 'a', 0.9], ['a', 'b', 0.075], ['a', 'c', 0.025],
  ['b', 'a', 0.15], ['b', 'b', 0.8], ['b', 'c', 0.05],
  ['c', 'a', 0.25], ['c', 'b', 0.25], ['c', 'c', 0.5],
];
const c = simulate('a', 5000, transitions);
console.log(`{'a': ${c.a || 0}, 'b': ${c.b || 0}, 'c': ${c.c || 0}}`);
