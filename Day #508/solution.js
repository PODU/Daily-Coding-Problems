// Approximate median: sample a fixed, sublinear subset (size independent of N),
// return the sample's median -> lands in the central half [N/4, 3N/4] w.h.p.
// Sampling+median: O(s log s), sublinear in N. (Rank shown only for the demo.)
'use strict';

const MASK = (1n << 64n) - 1n;
let state = 0x0123456789ABCDEFn; // fixed seed -> deterministic

function nextRand() {
    state = (state * 6364136223846793005n + 1442695040888963407n) & MASK;
    return Number(state >> 33n); // top 31 bits
}

function main() {
    const N = 1000;
    const SAMPLE_SIZE = 99; // fixed, independent of N (sublinear)

    const data = [];
    for (let i = 0; i < N; i++) data.push(i + 1);
    for (let i = N - 1; i > 0; i--) {
        const j = nextRand() % (i + 1);
        const t = data[i]; data[i] = data[j]; data[j] = t;
    }

    const sample = [];
    for (let i = 0; i < SAMPLE_SIZE; i++) {
        const idx = nextRand() % N;
        sample.push(data[idx]);
    }
    sample.sort((a, b) => a - b);
    const median = sample[Math.floor(SAMPLE_SIZE / 2)];

    let rank = 0;
    for (const v of data) if (v <= median) rank++;

    console.log(`Approximate median: ${median}`);
    console.log(`Rank: ${rank} (acceptable range: ${Math.floor(N / 4)} to ${Math.floor(3 * N / 4)})`);
}

main();
