// Day 640: Simulate a Markov chain and tally state visits.
// Approach: build outgoing-edge table, draw next state by cumulative prob.
// Time: O(num_steps * avg_out_degree), Space: O(states + edges).
// Note: output is stochastic; counts approximate the README sample (sum 5000).
function mulberry32(seed) {
  return function () {
    seed |= 0; seed = (seed + 0x6D2B79F5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function runMarkov(start, transitions, numSteps, seed = 42) {
  const adj = {};
  for (const [src, dst, p] of transitions) {
    (adj[src] = adj[src] || []).push([dst, p]);
  }
  const rand = mulberry32(seed);
  const counts = {};
  let cur = start;
  for (let i = 0; i < numSteps; i++) {
    counts[cur] = (counts[cur] || 0) + 1;
    const r = rand();
    let acc = 0;
    for (const [dst, p] of adj[cur]) {
      acc += p;
      if (r <= acc) { cur = dst; break; }
    }
  }
  return counts;
}

const transitions = [
  ['a', 'a', 0.9], ['a', 'b', 0.075], ['a', 'c', 0.025],
  ['b', 'a', 0.15], ['b', 'b', 0.8], ['b', 'c', 0.05],
  ['c', 'a', 0.25], ['c', 'b', 0.25], ['c', 'c', 0.5],
];
const counts = runMarkov('a', transitions, 5000);
const sorted = {};
for (const k of Object.keys(counts).sort()) sorted[k] = counts[k];
console.log(sorted); // e.g. { a: 3012, b: 1656, c: 332 }
