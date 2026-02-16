// Markov chain simulation: seeded LCG RNG (no external deps); O(steps*states) time O(states^2) space
// Counts state arrived at after each step (not initial state); total counts = num_steps = 5000
// Exact counts vary by seed; approx distribution: ~60% a, ~33% b, ~7% c

class SeededRandom {
    constructor(seed) { this.s = seed >>> 0; }
    next() {
        // Knuth multiplicative LCG
        this.s = (Math.imul(1664525, this.s) + 1013904223) >>> 0;
        return this.s / 0x100000000;
    }
}

const transitions = [
    ['a', 'a', 0.9],  ['a', 'b', 0.075], ['a', 'c', 0.025],
    ['b', 'a', 0.15], ['b', 'b', 0.8],   ['b', 'c', 0.05],
    ['c', 'a', 0.25], ['c', 'b', 0.25],  ['c', 'c', 0.5],
];

const trans = {};
for (const [f, t, p] of transitions) {
    if (!trans[f]) trans[f] = [];
    trans[f].push([t, p]);
}

const rng = new SeededRandom(42);
let state = 'a';
const counts = { a: 0, b: 0, c: 0 };
const numSteps = 5000;

for (let i = 0; i < numSteps; i++) {
    const r = rng.next();
    let cumulative = 0;
    for (const [to, prob] of trans[state]) {
        cumulative += prob;
        if (r < cumulative) {
            state = to;
            break;
        }
    }
    counts[state]++;
}

console.log(`{ 'a': ${counts.a}, 'b': ${counts.b}, 'c': ${counts.c} }`);
