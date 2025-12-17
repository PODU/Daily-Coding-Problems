// Day 755: Largest value path in a directed graph.
// Topological DP: dp[u][c] = max count of letter c on a path starting at u.
// Cycle -> value is infinite -> null. Time: O((n+m)*26), Space: O(n*26).
"use strict";

function largestPathValue(letters, edges) {
    const n = letters.length;
    const adj = Array.from({ length: n }, () => []);
    const indeg = new Array(n).fill(0);
    for (const [i, j] of edges) { adj[i].push(j); indeg[j]++; }

    const q = [];
    for (let i = 0; i < n; i++) if (indeg[i] === 0) q.push(i);
    const topo = [];
    let head = 0;
    while (head < q.length) {
        const u = q[head++];
        topo.push(u);
        for (const v of adj[u]) if (--indeg[v] === 0) q.push(v);
    }
    if (topo.length < n) return null;   // cycle

    const dp = Array.from({ length: n }, () => new Array(26).fill(0));
    for (let i = 0; i < n; i++) dp[i][letters.charCodeAt(i) - 65] = 1;

    let best = 0;
    for (let i = topo.length - 1; i >= 0; i--) {
        const u = topo[i];
        const uc = letters.charCodeAt(u) - 65;
        for (const v of adj[u])
            for (let c = 0; c < 26; c++) {
                const add = dp[v][c] + (c === uc ? 1 : 0);
                if (add > dp[u][c]) dp[u][c] = add;
            }
        for (let c = 0; c < 26; c++) best = Math.max(best, dp[u][c]);
    }
    return best;
}

const res = largestPathValue("ABACA", [[0, 1], [0, 2], [2, 3], [3, 4]]);
console.log(res === null ? "null" : res);  // 3
