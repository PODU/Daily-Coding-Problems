// Largest path value in directed graph: topo sort (Kahn) + DP dp[node][26]. Cycle -> null. Time O((n+m)*26), Space O(n*26).
'use strict';

// Graph "A" with edge (0,0) returns null (self-loop is a cycle).
function largestPathValue(colors, edges) {
    const n = colors.length;
    const adj = Array.from({ length: n }, () => []);
    const indeg = new Array(n).fill(0);
    for (const [a, b] of edges) {
        adj[a].push(b);
        indeg[b]++;
    }
    const dp = Array.from({ length: n }, () => new Array(26).fill(0));
    const q = [];
    for (let i = 0; i < n; i++) if (indeg[i] === 0) q.push(i);
    let head = 0, seen = 0, ans = 0;
    while (head < q.length) {
        const u = q[head++];
        seen++;
        const cu = colors.charCodeAt(u) - 65;
        dp[u][cu]++;
        ans = Math.max(ans, dp[u][cu]);
        for (const v of adj[u]) {
            for (let c = 0; c < 26; c++)
                if (dp[u][c] > dp[v][c]) dp[v][c] = dp[u][c];
            if (--indeg[v] === 0) q.push(v);
        }
    }
    if (seen < n) return null; // cycle
    return ans;
}

function main() {
    const colors = "ABACA";
    const edges = [[0, 1], [0, 2], [2, 3], [3, 4]];
    const result = largestPathValue(colors, edges);
    console.log(result === null ? "null" : result);
}

main();
