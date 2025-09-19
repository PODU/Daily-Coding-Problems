// Stream voting: track seen voters + candidate counts; report top-3 (count desc, id asc) or FRAUD on repeat.
// Time: O(n * c log c) over stream, Space: O(voters + candidates).
function processStream(stream) {
  const seen = new Set();
  const counts = new Map();
  for (const [voter, cand] of stream) {
    if (seen.has(voter)) {
      console.log(`Fraud: voter ${voter} voted more than once`);
      continue;
    }
    seen.add(voter);
    counts.set(cand, (counts.get(cand) || 0) + 1);
    const ranked = [...counts.entries()].sort((a, b) => {
      if (b[1] !== a[1]) return b[1] - a[1];
      return a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0;
    });
    const top3 = ranked.slice(0, 3).map(([cid]) => `'${cid}'`);
    console.log(`Top 3: [${top3.join(', ')}]`);
  }
}

const stream = [[1, 'A'], [2, 'B'], [3, 'A'], [4, 'C'], [5, 'B'], [1, 'A'], [6, 'A']];
processStream(stream);
