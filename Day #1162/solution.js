// Root tree, DFS subtree sizes; count non-root nodes with even subtree size (cuttable parent edges). O(n) time, O(n) space.
'use strict';

function solve(n, edges) {
    const adj = Array.from({ length: n + 1 }, () => []);
    for (const [a, b] of edges) {
        adj[a].push(b);
        adj[b].push(a);
    }
    let answer = 0;

    function dfs(u, parent) {
        let size = 1;
        for (const v of adj[u]) {
            if (v !== parent) size += dfs(v, u);
        }
        if (parent !== -1 && size % 2 === 0) answer++;
        return size;
    }

    dfs(1, -1);
    return answer;
}

const edges = [[1, 2], [1, 3], [3, 4], [3, 5], [4, 6], [4, 7], [4, 8]];
console.log(solve(8, edges));
