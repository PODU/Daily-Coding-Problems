// Day 1114 - Voting stream: top 3 candidates + fraud detection
// Approach: hash-map vote counts + set of seen voters (O(1) dup detection);
// top-3 via sort. Time: O(R + M log M), Space: O(V+M).

const stream = [
  [1, 'A'], [2, 'B'], [3, 'A'], [4, 'C'], [5, 'B'],
  [6, 'A'], [2, 'C'], [7, 'D'], [8, 'A'],
];

const counts = new Map();
const seen = new Set();
for (const [voter, cand] of stream) {
  if (seen.has(voter)) {
    console.log(`Fraud detected: voter ${voter} voted more than once`);
    continue;
  }
  seen.add(voter);
  counts.set(cand, (counts.get(cand) || 0) + 1);
}

const items = [...counts.entries()].sort((a, b) =>
  a[1] !== b[1] ? b[1] - a[1] : a[0] < b[0] ? -1 : 1);
const top = items.slice(0, 3).map(([c, n]) => `('${c}', ${n})`).join(", ");
console.log(`Top 3 candidates: [${top}]`);
