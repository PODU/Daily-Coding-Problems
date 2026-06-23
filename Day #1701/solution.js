// Stream voting: map vote counts + set of seen voters; duplicate voter = fraud.
// Top 3 by count (ties by candidate id). Time O(n + k log k), Space O(k+v).
function main() {
    const records = [[1,'A'],[2,'B'],[3,'A'],[4,'C'],[2,'A'],[5,'B'],[6,'A']];
    const counts = new Map();
    const seen = new Set();
    for (const [voter, cand] of records) {
        if (seen.has(voter)) {
            console.log(`Fraud detected: voter ${voter} voted more than once`);
            continue;
        }
        seen.add(voter);
        counts.set(cand, (counts.get(cand) || 0) + 1);
    }
    const top = [...counts.entries()]
        .sort((a, b) => b[1] - a[1] || (a[0] < b[0] ? -1 : 1))
        .slice(0, 3);
    console.log("Top 3 candidates: " + top.map(([c, n]) => `${c}(${n})`).join(", "));
}

main();
