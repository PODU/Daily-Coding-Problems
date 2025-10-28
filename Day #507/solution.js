// Streaming vote tally: Set of seen voters (duplicate -> fraud, vote dropped),
// Map candidate->count, top 3 computed on demand. Time O(n + k log k), Space O(n+k).
'use strict';

function main() {
    const stream = [
        ['v1', 'A'], ['v2', 'A'], ['v3', 'B'], ['v4', 'C'],
        ['v5', 'B'], ['v6', 'B'], ['v7', 'C'], ['v2', 'D'],
    ];

    const seen = new Set();
    const tally = new Map();

    for (const [voter, cand] of stream) {
        if (seen.has(voter)) {
            console.log(`Fraud detected: voter ${voter} voted more than once`);
            continue; // do not count fraudulent vote
        }
        seen.add(voter);
        tally.set(cand, (tally.get(cand) || 0) + 1);
    }

    const ordered = [...tally.entries()].sort((a, b) => {
        if (a[1] !== b[1]) return b[1] - a[1]; // higher votes first
        return a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0; // tie: candidate id ascending
    });
    const top = ordered.slice(0, 3);
    console.log('Top 3: ' + top.map(([c, n]) => `${c}(${n})`).join(', '));
}

main();
