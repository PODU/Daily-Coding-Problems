// Stream voting: Map candidate->count, Set of voters to detect fraud; top-3 via sort.
// Time: O(records) processing + O(C log C) reporting. Space: O(C + V).
'use strict';

function main() {
    const stream = [
        ["v1", "A"], ["v2", "B"], ["v3", "A"], ["v4", "C"],
        ["v5", "B"], ["v6", "A"], ["v7", "C"], ["v1", "B"],
    ];

    const counts = new Map();
    const seen = new Set();

    for (const [voter, cand] of stream) {
        if (seen.has(voter)) {
            console.log(`Fraud detected: voter ${voter}`);
            continue;
        }
        seen.add(voter);
        counts.set(cand, (counts.get(cand) || 0) + 1);
    }

    const ordered = [...counts.entries()].sort((a, b) => {
        if (a[1] !== b[1]) return b[1] - a[1];
        return a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0;
    });

    const top3 = ordered.slice(0, 3).map(e => e[0]).join(", ");
    console.log(`Top 3 candidates: ${top3}`);
}

main();
