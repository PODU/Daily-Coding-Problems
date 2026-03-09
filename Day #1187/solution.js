// Markov chain simulation: cumulative transition table, draw uniform RNG per step.
// Result is stochastic/approximate (fixed seed for reproducibility). Time O(steps), Space O(states^2).
'use strict';

// Simple seeded RNG (mulberry32) for reproducibility.
function mulberry32(seed) {
  return function () {
    seed |= 0; seed = (seed + 0x6D2B79F5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function simulate(start, numSteps, transitions, seed = 12345) {
  const raw = {};
  for (const [frm, to, prob] of transitions) {
    if (!raw[frm]) raw[frm] = [];
    raw[frm].push([prob, to]);
  }
  const table = {};
  for (const st of Object.keys(raw)) {
    let cum = 0;
    table[st] = raw[st].map(([prob, to]) => { cum += prob; return [cum, to]; });
  }

  const rng = mulberry32(seed);
  const counts = { a: 0, b: 0, c: 0 };
  let state = start;
  for (let i = 0; i < numSteps; i++) {
    const r = rng();
    for (const [cum, to] of table[state]) {
      if (r < cum) { state = to; break; }
    }
    counts[state]++;
  }
  return counts;
}

function main() {
  const transitions = [
    ['a', 'a', 0.9], ['a', 'b', 0.075], ['a', 'c', 0.025],
    ['b', 'a', 0.15], ['b', 'b', 0.8], ['b', 'c', 0.05],
    ['c', 'a', 0.25], ['c', 'b', 0.25], ['c', 'c', 0.5],
  ];
  const counts = simulate('a', 5000, transitions);
  console.log(`{'a': ${counts.a}, 'b': ${counts.b}, 'c': ${counts.c}}`);
}

main();
